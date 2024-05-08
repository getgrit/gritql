#![cfg_attr(not(test), warn(unused_crate_dependencies))]
use marzano_cli::commands::run_command;
use marzano_cli::error::GoodError;
// We always instrument

#[cfg(feature = "grit_tracing")]
use marzano_util::base64;
#[cfg(feature = "grit_tracing")]
use opentelemetry::{global, KeyValue};
#[cfg(feature = "grit_tracing")]
use opentelemetry_otlp::WithExportConfig;
#[cfg(feature = "grit_tracing")]
use opentelemetry_sdk::propagation::TraceContextPropagator;
#[cfg(feature = "grit_tracing")]
use opentelemetry_sdk::trace::Tracer;
#[cfg(feature = "grit_tracing")]
use opentelemetry_sdk::{trace, Resource};
#[cfg(feature = "grit_tracing")]
use std::collections::HashMap;
#[cfg(feature = "grit_tracing")]
use tracing::Instrument;
#[cfg(feature = "grit_tracing")]
use tracing::{event, span, Level};
#[cfg(feature = "grit_tracing")]
#[allow(unused_imports)]
use tracing_subscriber::prelude::*;
#[cfg(feature = "grit_tracing")]
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use anyhow::Result;
#[cfg(feature = "workflows_v2")]
mod workflows;

#[cfg(feature = "grit_tracing")]
fn get_otel_key(env_name: &str) -> Option<String> {
    match std::env::var(env_name) {
        Ok(key) => {
            if key.is_empty() {
                None
            } else {
                Some(key)
            }
        }
        Err(_) => None,
    }
}

#[cfg(feature = "grit_tracing")]
fn get_otel_setup() -> Result<Option<Tracer>> {
    use anyhow::bail;

    let mut exporter = opentelemetry_otlp::new_exporter()
        .http()
        .with_http_client(reqwest::Client::default())
        .with_timeout(std::time::Duration::from_millis(500));

    let grafana_key = get_otel_key("GRAFANA_OTEL_KEY");
    let honeycomb_key = get_otel_key("HONEYCOMB_OTEL_KEY");
    let baselime_key = get_otel_key("BASELIME_OTEL_KEY");
    let hyperdx_key = get_otel_key("HYPERDX_OTEL_KEY");

    match (grafana_key, honeycomb_key, baselime_key, hyperdx_key) {
        (None, None, None, None) => {
            // NOTE: we don't include tracing in released builds, so this won't appear
            eprintln!("No OTLP key found, tracing will be disabled");
            return Ok(None);
        }
        (Some(grafana_key), None, None, None) => {
            let instance_id = "665534";
            let encoded =
                base64::encode_from_string(format!("{}:{}", instance_id, grafana_key).as_str())?;
            exporter = exporter
                .with_endpoint("https://otlp-gateway-prod-us-central-0.grafana.net/otlp")
                .with_headers(HashMap::from([(
                    "Authorization".into(),
                    format!("Basic {}", encoded),
                )]));
        }
        (None, Some(honeycomb_key), None, None) => {
            exporter = exporter
                .with_endpoint("https://api.honeycomb.io")
                .with_headers(HashMap::from([("x-honeycomb-team".into(), honeycomb_key)]));
        }
        (None, None, Some(baselime_key), None) => {
            exporter = exporter
                .with_endpoint("https://otel.baselime.io/v1/")
                .with_headers(HashMap::from([
                    ("x-api-key".into(), baselime_key),
                    ("x-baselime-dataset".into(), "otel".into()),
                ]));
        }
        (None, None, None, Some(hyperdx_key)) => {
            exporter = exporter
                .with_endpoint("https://in-otel.hyperdx.io")
                .with_headers(HashMap::from([("authorization".into(), hyperdx_key)]));
        }
        _ => bail!("multiple OTLP keys found"),
    }

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(
            trace::config().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                "grit_marzano",
            )])),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;

    Ok(Some(tracer))
}

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(feature = "grit_tracing")]
    {
        let tracer = get_otel_setup()?;

        if let Some(tracer) = tracer {
            let env_filter = EnvFilter::try_from_default_env()
                .unwrap_or(EnvFilter::new("TRACE"))
                // We don't want to trace the tracing library itself
                .add_directive("hyper::proto=off".parse().unwrap());

            let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
            let subscriber = Registry::default().with(env_filter).with(telemetry);

            global::set_text_map_propagator(TraceContextPropagator::new());
            tracing::subscriber::set_global_default(subscriber)
                .expect("setting tracing default failed");

            let root_span = span!(Level::INFO, "grit_marzano.cli_command",);

            let _res = async move {
                event!(Level::INFO, "starting the CLI!");

                let res = run_command().await;

                event!(Level::INFO, "ending the CLI!");

                res
            }
            .instrument(root_span)
            .await;

            opentelemetry::global::shutdown_tracer_provider();

            return Ok(());
        }
    }
    let subscriber = tracing::subscriber::NoSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).expect("setting tracing default failed");

    let res = run_command().await;
    if let Err(ref e) = res {
        if let Some(good) = e.downcast_ref::<GoodError>() {
            if let Some(msg) = &good.message {
                println!("{}", msg);
            }
            std::process::exit(1);
        }
    }
    return res;
}
