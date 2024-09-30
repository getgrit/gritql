pub(crate) mod apply;

pub(crate) mod apply_pattern;

pub(crate) mod auth;
pub(crate) mod auth_login;
pub(crate) mod auth_logout;
pub(crate) mod auth_refresh;
pub(crate) mod auth_token;

pub(crate) mod doctor;
pub(crate) mod init;
pub(crate) mod install;
pub(crate) mod list;
pub(crate) mod lsp;

pub(crate) mod check;

pub(crate) mod parse;
pub(crate) mod patterns;
pub(crate) mod patterns_list;
pub(crate) mod patterns_test;
pub(crate) mod plumbing;
pub(crate) mod version;

#[cfg(feature = "workflows_v2")]
pub(crate) mod apply_migration;
#[cfg(feature = "workflows_v2")]
pub(crate) mod workflows;
#[cfg(feature = "workflows_v2")]
pub(crate) mod workflows_list;
#[cfg(feature = "workflows_v2")]
pub(crate) mod workflows_upload;
#[cfg(feature = "workflows_v2")]
pub(crate) mod workflows_watch;

use crate::error::GoodError;

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
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

#[cfg(feature = "docgen")]
pub(crate) mod docgen;
mod filters;

use crate::{
    analytics::{
        is_telemetry_disabled, is_telemetry_foregrounded, AnalyticsEvent, AnalyticsEventName,
        CompletedEvent, ErroredEvent,
    },
    flags::{GlobalFormatFlags, OutputFormat},
    updater::Updater,
};
use anyhow::Result;
use apply::ApplyArgs;
use auth::{Auth, AuthCommands};
use check::CheckArg;
use clap::Parser;
use clap::Subcommand;
use doctor::DoctorArgs;
use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;
use init::InitArgs;
use install::InstallArgs;
use list::ListArgs;
use log::LevelFilter;
use lsp::LspArgs;
use marzano_messenger::emit::ApplyDetails;
use parse::ParseArgs;
use patterns::{PatternCommands, Patterns};
use plumbing::PlumbingArgs;
use serde::Serialize;
use std::io::Write;
use std::process::{ChildStdin, Command, Stdio};
use std::time::Instant;
use std::{fmt, process::Child};
use version::VersionArgs;

#[cfg(feature = "workflows_v2")]
use crate::commands::workflows::{WorkflowCommands, Workflows};
#[cfg(feature = "workflows_v2")]
use workflows_list::run_list_workflows;
#[cfg(feature = "workflows_v2")]
use workflows_upload::run_upload_workflow;
#[cfg(feature = "workflows_v2")]
use workflows_watch::run_watch_workflow;

#[cfg(feature = "docgen")]
use crate::commands::docgen::{run_docgen, DocGenArgs};

use self::{
    apply::run_apply,
    auth_login::run_login,
    auth_logout::run_logout,
    auth_refresh::run_refresh_auth,
    auth_token::run_get_token,
    check::run_check,
    doctor::run_doctor,
    init::run_init,
    install::run_install,
    list::run_list_all,
    lsp::run_lsp,
    parse::run_parse,
    patterns::{run_patterns_describe, run_patterns_edit},
    patterns_list::run_patterns_list,
    patterns_test::run_patterns_test,
    plumbing::run_plumbing,
    version::run_version,
};

#[derive(Subcommand, Debug, Serialize)]
pub enum Commands {
    /// Check the current directory for pattern violations.
    Check(CheckArg),
    /// List everything that can be applied to the current directory.
    List(ListArgs),
    /// Apply a pattern or migration to a set of files
    Apply(ApplyArgs),
    /// Start a language server for Grit.
    #[clap(hide = true)]
    Lsp(LspArgs),
    /// Print diagnostic information about the current environment
    Doctor(DoctorArgs),
    /// Authentication commands, run `grit auth --help` for more information
    #[clap(name = "auth")]
    Auth(Auth),
    /// Install supporting binaries
    Install(InstallArgs),
    /// Install grit modules
    Init(InitArgs),
    /// Hidden command for parsing input files, consumed by provolone
    #[clap(name = "parse", hide = true)]
    Parse(ParseArgs),
    /// Workflow commands, run `grit workflows --help` for more information
    #[cfg(feature = "workflows_v2")]
    #[clap(name = "workflows")]
    Workflows(Workflows),
    /// Patterns commands, run `grit patterns --help` for more information
    #[clap(name = "patterns")]
    Patterns(Patterns),
    /// Plumbing subcommands for easy machine integration
    #[clap(subcommand, name = "plumbing", hide = true)]
    Plumbing(PlumbingArgs),
    /// Display version information about the CLI and agents
    Version(VersionArgs),
    /// Generate documentation for the Grit CLI (internal use only)
    #[cfg(feature = "docgen")]
    #[clap(hide = true)]
    Docgen(DocGenArgs),
    /// Server-only commands (for Grit Cloud)
    #[cfg(feature = "server")]
    Server(cli_server::commands::ServerArgs),
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Commands::Apply(_) => write!(f, "apply"),
            Commands::Check(_) => write!(f, "check"),
            Commands::List(_) => write!(f, "list"),
            Commands::Lsp(_) => write!(f, "lsp"),
            Commands::Doctor(_) => write!(f, "doctor"),
            Commands::Auth(arg) => match arg.auth_commands {
                AuthCommands::Login(_) => write!(f, "auth login"),
                AuthCommands::Logout(_) => write!(f, "auth logout"),
                AuthCommands::GetToken(_) => write!(f, "auth get-token"),
                AuthCommands::Refresh(_) => write!(f, "auth refresh"),
            },
            Commands::Install(_) => write!(f, "install"),
            Commands::Init(_) => write!(f, "init"),
            Commands::Parse(_) => write!(f, "parse"),
            Commands::Patterns(arg) => match arg.patterns_commands {
                PatternCommands::List(_) => write!(f, "patterns list"),
                PatternCommands::Test(_) => write!(f, "patterns test"),
                PatternCommands::Edit(_) => write!(f, "patterns edit"),
                PatternCommands::Describe(_) => write!(f, "patterns describe"),
            },
            #[cfg(feature = "workflows_v2")]
            Commands::Workflows(arg) => match arg.workflows_commands {
                WorkflowCommands::List(_) => write!(f, "workflows list"),
                WorkflowCommands::Watch(_) => write!(f, "workflows watch"),
                WorkflowCommands::Upload(_) => write!(f, "workflows upload"),
            },
            Commands::Plumbing(_) => write!(f, "plumbing"),
            Commands::Version(_) => write!(f, "version"),
            #[cfg(feature = "docgen")]
            Commands::Docgen(_) => write!(f, "docgen"),
            #[cfg(feature = "server")]
            Commands::Server(_) => write!(f, "server"),
        }
    }
}
#[derive(Parser)]
#[clap(
    bin_name = "grit",
    name = "grit",
    author,
    about = if cfg!(feature = "server") { "Grit, server edition" } else { "Software maintenance on autopilot, from grit.io" },
    after_help = "For help with a specific command, run `grit help <command>`.",
    version
)]

struct App {
    #[clap(subcommand)]
    command: Commands,
    #[clap(flatten)]
    pub format_flags: GlobalFormatFlags,
}

fn maybe_spawn_analytics_worker(
    command: &Commands,
    args: &[String],
    updater: &Updater,
) -> Result<Option<Child>> {
    if is_telemetry_disabled() {
        return Ok(None);
    }

    if let Commands::Plumbing(PlumbingArgs::Analytics { .. }) = command {
        return Ok(None);
    }

    let exe = std::env::current_exe()?;
    let mut cmd = Command::new(exe);

    cmd.arg("plumbing");
    cmd.arg("analytics");

    if let Some(auth) = updater.get_auth() {
        cmd.arg("--user-id");
        cmd.arg(&auth.get_user_id()?);
    }

    let installation_id = updater.installation_id;
    cmd.arg("--installation-id");
    cmd.arg(installation_id.to_string());

    cmd.arg("--command")
        .arg(command.to_string())
        .arg("--args")
        .arg(args.join(" "))
        .stdin(Stdio::piped());

    if is_telemetry_foregrounded() {
        cmd.arg("--log-level=info");
    } else {
        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::null());
    }

    let child = cmd.spawn()?;

    Ok(Some(child))
}

fn write_analytics_event(
    analytics_worker: Option<&mut ChildStdin>,
    analytics_event: &AnalyticsEvent,
) {
    let serialized_name = serde_json::to_string(&AnalyticsEventName::from(analytics_event));
    let serialized_event = serde_json::to_string(&analytics_event);
    match (analytics_worker, serialized_name, serialized_event) {
        (Some(analytics_worker), Ok(serialized_name), Ok(serialized_event)) => {
            let data = format!("{}\t{}\n", serialized_name, serialized_event);
            let res = analytics_worker.write_all(data.as_bytes());
            if let Err(e) = res {
                log::info!("Failed to write to analytics worker: {:?}", e);
            }
        }
        (None, _, _) => {
            // No analytics worker to send event to, do nothing
        }
        (worker, name_err, event_err) => {
            log::info!(
                "Failed to serialize analytics event: {:?} {:?} {:?}",
                worker,
                name_err,
                event_err
            );
        }
    }
}

fn setup_env_logger(app: &App, multi: &MultiProgress) {
    let format: OutputFormat = (&app.format_flags).into();
    let mut logger = env_logger::Builder::new();

    let log_level = app.format_flags.log_level.unwrap_or(match &app.command {
        Commands::Lsp(_) => LevelFilter::Off,
        Commands::Plumbing(_) => LevelFilter::Off,
        _ => LevelFilter::Info,
    });

    logger.filter_level(log_level);
    logger.target(match format {
        OutputFormat::Standard => env_logger::Target::Stdout,
        OutputFormat::Transformed => env_logger::Target::Stderr,
        OutputFormat::Json | OutputFormat::Jsonl => env_logger::Target::Stderr,
        #[cfg(feature = "remote_redis")]
        OutputFormat::Redis => env_logger::Target::Stderr,
        #[cfg(feature = "remote_pubsub")]
        OutputFormat::PubSub => env_logger::Target::Stderr,
        #[cfg(feature = "server")]
        OutputFormat::Combined => env_logger::Target::Stderr,
    });
    if !matches!(app.command, Commands::Plumbing(_)) {
        logger.format(|buf, record| writeln!(buf, "{}", record.args()));
    };
    let logger = logger.build();
    if let Err(e) = LogWrapper::new(multi.clone(), logger).try_init() {
        eprintln!("Failed to initialize logger: {:?}", e);
    }
}

async fn run_command(_use_tracing: bool) -> Result<()> {
    let app = App::parse();
    // Use this *only* for analytics, not for any other purpose.
    let analytics_args = std::env::args().collect::<Vec<_>>();

    // Create and save installation ID if needed
    let mut updater = Updater::from_current_bin().await?;
    updater.dump().await?;

    let mut analytics_child =
        match maybe_spawn_analytics_worker(&app.command, &analytics_args, &updater) {
            Err(_e) => {
                log::info!("Failed to start the analytics worker process");
                // We failed to start the analytics worker process
                None
            }
            Ok(None) => None,
            Ok(Some(child)) => Some(child),
        };

    let multi = MultiProgress::new();

    #[cfg(not(feature = "grit_tracing"))]
    setup_env_logger(&app, &multi);
    #[cfg(feature = "grit_tracing")]
    if !_use_tracing {
        setup_env_logger(&app, &multi);
    } else if let Err(e) = tracing_log::log_tracer::Builder::new()
        .ignore_all(vec!["rustls", "tonic", "mio", "hyper"])
        .with_max_level(LevelFilter::Debug)
        .init()
    {
        eprintln!("Failed to initialize LogTracer: {:?}", e);
        setup_env_logger(&app, &multi)
    }

    let command = app.command.to_string();
    let mut apply_details = ApplyDetails {
        matched: 0,
        rewritten: 0,
        named_pattern: None,
    };
    let start = Instant::now();

    write_analytics_event(
        analytics_child.as_mut().map(|c| c.stdin.as_mut().unwrap()),
        &AnalyticsEvent::from_cmd(&app.command),
    );

    match &app.command {
        Commands::Install(_) => {}
        Commands::Plumbing(_) => {}
        _ => {
            updater.check_for_update().await?;
        }
    };

    #[cfg(feature = "grit_tracing")]
    let cmd_span = span!(
        Level::INFO,
        "grit_marzano.run_command",
        "grit.command" = command.as_str(),
        "grit.args" = analytics_args.join(" ")
    );

    let runner = async move {
        let res = match app.command {
            Commands::Apply(arg) => {
                run_apply(arg, multi, &mut apply_details, &app.format_flags).await
            }
            Commands::Check(arg) => run_check(arg, &app.format_flags, multi, false, None).await,
            Commands::List(arg) => run_list_all(&arg, &app.format_flags).await,
            Commands::Doctor(arg) => run_doctor(arg).await,
            Commands::Auth(arg) => match arg.auth_commands {
                AuthCommands::Login(arg) => run_login(arg).await,
                AuthCommands::Logout(arg) => run_logout(arg).await,
                AuthCommands::GetToken(arg) => run_get_token(arg).await,
                AuthCommands::Refresh(arg) => run_refresh_auth(arg).await,
            },
            Commands::Lsp(arg) => run_lsp(arg).await,
            Commands::Install(arg) => run_install(arg).await,
            Commands::Init(arg) => run_init(arg).await,
            Commands::Parse(arg) => run_parse(arg, app.format_flags, None).await,
            Commands::Patterns(arg) => match arg.patterns_commands {
                PatternCommands::List(arg) => run_patterns_list(arg, app.format_flags).await,
                PatternCommands::Test(arg) => run_patterns_test(arg, app.format_flags).await,
                PatternCommands::Edit(arg) => run_patterns_edit(arg).await,
                PatternCommands::Describe(arg) => run_patterns_describe(arg).await,
            },
            #[cfg(feature = "workflows_v2")]
            Commands::Workflows(arg) => match arg.workflows_commands {
                WorkflowCommands::List(arg) => run_list_workflows(&arg, &app.format_flags).await,
                WorkflowCommands::Watch(arg) => run_watch_workflow(&arg, &app.format_flags).await,
                WorkflowCommands::Upload(arg) => run_upload_workflow(&arg, &app.format_flags)
                    .await
                    .map(|_| ()),
            },
            Commands::Plumbing(arg) => {
                run_plumbing(arg, multi, &mut apply_details, app.format_flags).await
            }
            Commands::Version(arg) => run_version(arg).await,
            #[cfg(feature = "docgen")]
            Commands::Docgen(arg) => run_docgen(arg).await,
            #[cfg(feature = "server")]
            Commands::Server(arg) => cli_server::commands::run_server_command(arg).await,
        };
        let elapsed = start.elapsed();
        let details = if command == "apply" {
            Some(apply_details)
        } else {
            None
        };

        let final_analytics_event = match res {
            Ok(_) => AnalyticsEvent::Completed(CompletedEvent::from_res(elapsed, details)),

            Err(_) => AnalyticsEvent::Errored(ErroredEvent::from_elapsed(elapsed)),
        };

        write_analytics_event(
            analytics_child.as_mut().map(|c| c.stdin.as_mut().unwrap()),
            &final_analytics_event,
        );

        // If we are in the foreground, wait for the analytics worker to finish
        if is_telemetry_foregrounded() {
            if let Some(mut child) = analytics_child {
                log::info!("Waiting for analytics worker to finish");
                let res = child.wait();
                if let Err(e) = res {
                    log::info!("Failed to wait for analytics worker: {:?}", e);
                }
            }
        }

        res
    };

    #[cfg(feature = "grit_tracing")]
    let res = runner.instrument(cmd_span).await;
    #[cfg(not(feature = "grit_tracing"))]
    let res = runner.await;

    res
}

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
fn get_otel_setup() -> Result<Option<(Tracer, opentelemetry_sdk::logs::LoggerProvider)>> {
    let grafana_key = get_otel_key("GRAFANA_OTEL_KEY");
    let honeycomb_key = get_otel_key("HONEYCOMB_OTEL_KEY");
    let baselime_key = get_otel_key("BASELIME_OTEL_KEY");
    let hyperdx_key = get_otel_key("HYPERDX_OTEL_KEY");

    let env = get_otel_key("GRIT_DEPLOYMENT_ENV").unwrap_or_else(|| "prod".to_string());

    let (endpoint, headers) = match (grafana_key, honeycomb_key, baselime_key, hyperdx_key) {
        (None, None, None, None) => {
            if let Some(endpoint) = get_otel_key("OTEL_EXPORTER_OTLP_ENDPOINT") {
                eprintln!(
                    "No explicit OTLP key found, using default OTLP endpoint: {}",
                    endpoint
                );
                (endpoint, HashMap::new())
            } else {
                #[cfg(feature = "server")]
                eprintln!("No OTLP key found, tracing will be disabled");
                return Ok(None);
            }
        }

        (Some(grafana_key), _, _, _) => {
            let instance_id = "665534";
            let encoded =
                base64::encode_from_string(format!("{}:{}", instance_id, grafana_key).as_str())?;
            let endpoint = "https://otlp-gateway-prod-us-central-0.grafana.net/otlp".to_string();
            let headers =
                HashMap::from([("Authorization".to_string(), format!("Basic {}", encoded))]);
            eprintln!("Using Grafana OTLP key for {}", env);
            (endpoint, headers)
        }
        (_, Some(honeycomb_key), _, _) => {
            let endpoint = "https://api.honeycomb.io".to_string();
            let headers = HashMap::from([("x-honeycomb-team".to_string(), honeycomb_key)]);
            eprintln!("Using Honeycomb OTLP key for {}", env);
            (endpoint, headers)
        }
        (_, _, Some(baselime_key), _) => {
            let endpoint = "https://otel.baselime.io/v1/".to_string();
            let headers = HashMap::from([
                ("x-api-key".to_string(), baselime_key),
                ("x-baselime-dataset".to_string(), "otel".to_string()),
            ]);
            eprintln!("Using Baselime OTLP key for {}", env);
            (endpoint, headers)
        }
        (_, _, _, Some(hyperdx_key)) => {
            let endpoint = "https://in-otel.hyperdx.io".to_string();
            let headers = HashMap::from([("authorization".to_string(), hyperdx_key)]);
            eprintln!("Using HyperDX OTLP key for {}", env);
            (endpoint, headers)
        }
    };

    let client = reqwest::Client::new();

    let logger = opentelemetry_otlp::new_pipeline()
        .logging()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .http()
                .with_http_client(client.clone())
                .with_timeout(std::time::Duration::from_millis(500))
                .with_endpoint(endpoint.clone())
                .with_headers(headers.clone()),
        )
        .with_log_config(
            opentelemetry_sdk::logs::config().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                format!("{}_grit_marzano", env),
            )])),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .http()
                .with_http_client(client)
                .with_timeout(std::time::Duration::from_millis(500))
                .with_endpoint(endpoint)
                .with_headers(headers),
        )
        .with_trace_config(
            trace::config().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                format!("{}_grit_marzano", env),
            )])),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;

    let logger_provider = logger.provider().unwrap();

    Ok(Some((tracer, logger_provider)))
}

pub async fn run_command_with_tracing() -> Result<()> {
    #[cfg(feature = "grit_tracing")]
    {
        let otel = get_otel_setup()?;

        if let Some((tracer, logger)) = otel {
            let env_filter = EnvFilter::try_from_default_env()
                .unwrap_or(EnvFilter::new("TRACE"))
                // Exclude noisy tokio stuff "h2::proto::streams::prioritize
                .add_directive("h2=off".parse().unwrap())
                // This is also noisy
                .add_directive("axum::serve=DEBUG".parse().unwrap())
                // We don't want to trace the tracing library itself
                .add_directive("hyper=off".parse().unwrap());

            let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
            let subscriber = Registry::default().with(env_filter).with(telemetry);

            global::set_text_map_propagator(TraceContextPropagator::new());
            tracing::subscriber::set_global_default(subscriber)
                .expect("setting tracing default failed");

            // let log_provider = logger.provider().unwrap();
            let trace_appender_layer = OpenTelemetryTracingBridge::new(&logger);
            // let logger_provider = opentelemetry::logs::NoopLoggerProvider::new();
            // let logger_provider = global::logger_provider();
            // let logger_layer = OpenTelemetryTracingBridge::new(&logger_provider);

            // let root_span = span!(Level::INFO, "grit_marzano.cli_command",);

            // let res = async move {
            //     event!(Level::INFO, "starting the CLI!");

            //     let res = run_command(true).await;

            //     event!(Level::INFO, "ending the CLI!");

            //     res
            // }
            // .instrument(root_span)
            // .await;

            opentelemetry::global::shutdown_tracer_provider();

            return Ok(());
        }
    }
    let subscriber = tracing::subscriber::NoSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).expect("setting tracing default failed");

    let res = run_command(false).await;
    if let Err(ref e) = res {
        if let Some(good) = e.downcast_ref::<GoodError>() {
            if let Some(msg) = &good.message {
                // grit-ignore no_println_in_core: This is an outer error message
                println!("{}", msg);
            }
            std::process::exit(1);
        }
    }
    res
}
