/// This file is a modified version of the opentelemetry-appender-tracing crate.
/// We need to use this to bridge between the opentelemetry and the tracing crate.
/// The opentelemetry-appender-tracing crate has version mismatch issues.
/// Source: https://github.com/open-telemetry/opentelemetry-rust/blob/v0.22.1/opentelemetry-appender-tracing/src/layer.rs
///
/// Includes the workaround for https://github.com/open-telemetry/opentelemetry-rust/issues/1378
/// From https://github.com/open-telemetry/opentelemetry-rust/pull/1394
///
/// Copyright The OpenTelemetry Authors
/// Modified by Iuvo AI, Inc.
use opentelemetry::{
    logs::{AnyValue, LogRecord, Logger, LoggerProvider, Severity},
    Key,
};
use std::borrow::Cow;
use tracing_subscriber::registry::LookupSpan;

use tracing::{field::Field, Level, Metadata, Subscriber};
#[cfg(feature = "experimental_metadata_attributes")]
use tracing_log::NormalizeEvent;
use tracing_subscriber::Layer;

const INSTRUMENTATION_LIBRARY_NAME: &str = "opentelemetry-appender-tracing";

/// Visitor to record the fields from the event record.
#[derive(Default)]
struct EventVisitor {
    log_record_attributes: Vec<(Key, AnyValue)>,
    log_record_body: Option<AnyValue>,
}

/// Logs from the log crate have duplicated attributes that we removed here.
#[cfg(feature = "experimental_metadata_attributes")]
fn is_duplicated_metadata(field: &'static str) -> bool {
    field
        .strip_prefix("log.")
        .map(|remainder| matches!(remainder, "file" | "line" | "module_path" | "target"))
        .unwrap_or(false)
}

#[cfg(feature = "experimental_metadata_attributes")]
fn get_filename(filepath: &str) -> &str {
    if let Some((_, filename)) = filepath.rsplit_once('/') {
        return filename;
    }
    if let Some((_, filename)) = filepath.rsplit_once('\\') {
        return filename;
    }
    filepath
}

impl EventVisitor {
    fn visit_metadata(&mut self, meta: &Metadata) {
        self.log_record_attributes
            .push(("name".into(), meta.name().into()));

        #[cfg(feature = "experimental_metadata_attributes")]
        self.visit_experimental_metadata(meta);
    }

    #[cfg(feature = "experimental_metadata_attributes")]
    fn visit_experimental_metadata(&mut self, meta: &Metadata) {
        self.log_record_attributes
            .push(("log.target".into(), meta.target().to_owned().into()));

        if let Some(module_path) = meta.module_path() {
            self.log_record_attributes
                .push(("code.namespace".into(), module_path.to_owned().into()));
        }

        if let Some(filepath) = meta.file() {
            self.log_record_attributes
                .push(("code.filepath".into(), filepath.to_owned().into()));
            self.log_record_attributes.push((
                "code.filename".into(),
                get_filename(filepath).to_owned().into(),
            ));
        }

        if let Some(line) = meta.line() {
            self.log_record_attributes
                .push(("code.lineno".into(), line.into()));
        }
    }

    fn push_to_otel_log_record(self, log_record: &mut LogRecord) {
        log_record.body = self.log_record_body;
        log_record.attributes = Some(self.log_record_attributes);
    }
}

impl tracing::field::Visit for EventVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        #[cfg(feature = "experimental_metadata_attributes")]
        if is_duplicated_metadata(field.name()) {
            return;
        }
        if field.name() == "message" {
            self.log_record_body = Some(format!("{value:?}").into());
        } else {
            self.log_record_attributes
                .push((field.name().into(), format!("{value:?}").into()));
        }
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        #[cfg(feature = "experimental_metadata_attributes")]
        if is_duplicated_metadata(field.name()) {
            return;
        }
        self.log_record_attributes
            .push((field.name().into(), value.to_owned().into()));
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.log_record_attributes
            .push((field.name().into(), value.into()));
    }

    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        self.log_record_attributes
            .push((field.name().into(), value.into()));
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        #[cfg(feature = "experimental_metadata_attributes")]
        if is_duplicated_metadata(field.name()) {
            return;
        }
        self.log_record_attributes
            .push((field.name().into(), value.into()));
    }

    // TODO: Remaining field types from AnyValue : Bytes, ListAny, Boolean
}

pub struct OpenTelemetryTracingBridge<P, L>
where
    P: LoggerProvider<Logger = L> + Send + Sync,
    L: Logger + Send + Sync,
{
    logger: L,
    _phantom: std::marker::PhantomData<P>, // P is not used.
}

impl<P, L> OpenTelemetryTracingBridge<P, L>
where
    P: LoggerProvider<Logger = L> + Send + Sync,
    L: Logger + Send + Sync,
{
    pub fn new(provider: &P) -> Self {
        OpenTelemetryTracingBridge {
            logger: provider.versioned_logger(
                INSTRUMENTATION_LIBRARY_NAME,
                Some(Cow::Borrowed(env!("CARGO_PKG_VERSION"))),
                None,
                None,
            ),
            _phantom: Default::default(),
        }
    }
}

impl<S, P, L> Layer<S> for OpenTelemetryTracingBridge<P, L>
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    P: LoggerProvider<Logger = L> + Send + Sync + 'static,
    L: Logger + Send + Sync + 'static,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        #[cfg(feature = "experimental_metadata_attributes")]
        let normalized_meta = event.normalized_metadata();
        #[cfg(feature = "experimental_metadata_attributes")]
        let meta = normalized_meta.as_ref().unwrap_or_else(|| event.metadata());

        #[cfg(not(feature = "experimental_metadata_attributes"))]
        let meta = event.metadata();

        let mut log_record: LogRecord = LogRecord::default();
        log_record.severity_number = Some(severity_of_level(meta.level()));
        log_record.severity_text = Some(meta.level().to_string().into());

        // Extract the trace_id & span_id from the opentelemetry extension.
        set_trace_context(&mut log_record, &_ctx);

        // Not populating ObservedTimestamp, instead relying on OpenTelemetry
        // API to populate it with current time.

        let mut visitor = EventVisitor::default();
        visitor.visit_metadata(meta);
        // Visit fields.
        event.record(&mut visitor);
        visitor.push_to_otel_log_record(&mut log_record);

        self.logger.emit(log_record);
    }

    #[cfg(feature = "logs_level_enabled")]
    fn event_enabled(
        &self,
        _event: &Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        let severity = severity_of_level(_event.metadata().level());
        self.logger
            .event_enabled(severity, _event.metadata().target())
    }
}

fn set_trace_context<S>(log_record: &mut LogRecord, ctx: &tracing_subscriber::layer::Context<'_, S>)
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    use opentelemetry::{
        logs::TraceContext,
        trace::{SpanContext, TraceFlags, TraceState},
    };

    if let Some((trace_id, span_id)) = ctx.lookup_current().and_then(|span| {
        span.extensions()
            .get::<tracing_opentelemetry::OtelData>()
            .and_then(|ext| ext.builder.trace_id.zip(ext.builder.span_id))
    }) {
        log_record.trace_context = Some(TraceContext::from(&SpanContext::new(
            trace_id,
            span_id,
            TraceFlags::default(),
            false,
            TraceState::default(),
        )));
    }
}

const fn severity_of_level(level: &Level) -> Severity {
    match *level {
        Level::TRACE => Severity::Trace,
        Level::DEBUG => Severity::Debug,
        Level::INFO => Severity::Info,
        Level::WARN => Severity::Warn,
        Level::ERROR => Severity::Error,
    }
}

#[cfg(test)]
mod tests {
    use crate::layer;
    use opentelemetry::logs::Severity;
    use opentelemetry::trace::TracerProvider as _;
    use opentelemetry::trace::{TraceContextExt, TraceFlags, Tracer};
    use opentelemetry::{logs::AnyValue, Key};
    use opentelemetry_sdk::logs::LoggerProvider;
    use opentelemetry_sdk::testing::logs::InMemoryLogsExporter;
    use opentelemetry_sdk::trace::{config, Sampler, TracerProvider};
    use tracing::error;
    use tracing_subscriber::layer::SubscriberExt;

    // cargo test --features=testing
    #[test]
    fn tracing_appender_standalone() {
        // Arrange
        let exporter: InMemoryLogsExporter = InMemoryLogsExporter::default();
        let logger_provider = LoggerProvider::builder()
            .with_simple_exporter(exporter.clone())
            .build();

        let layer = layer::OpenTelemetryTracingBridge::new(&logger_provider);
        let subscriber = tracing_subscriber::registry().with(layer);

        // avoiding setting tracing subscriber as global as that does not
        // play well with unit tests.
        let _guard = tracing::subscriber::set_default(subscriber);

        // Act
        error!(name: "my-event-name", target: "my-system", event_id = 20, user_name = "otel", user_email = "otel@opentelemetry.io");
        logger_provider.force_flush();

        // Assert TODO: move to helper methods
        let exported_logs = exporter
            .get_emitted_logs()
            .expect("Logs are expected to be exported.");
        assert_eq!(exported_logs.len(), 1);
        let log = exported_logs
            .first()
            .expect("Atleast one log is expected to be present.");

        // Validate common fields
        assert_eq!(log.instrumentation.name, "opentelemetry-appender-tracing");
        assert_eq!(log.record.severity_number, Some(Severity::Error));

        // Validate trace context is none.
        assert!(log.record.trace_context.is_none());

        // Validate attributes
        let attributes: Vec<(Key, AnyValue)> = log
            .record
            .attributes
            .clone()
            .expect("Attributes are expected");
        #[cfg(not(feature = "experimental_metadata_attributes"))]
        assert_eq!(attributes.len(), 4);
        #[cfg(feature = "experimental_metadata_attributes")]
        assert_eq!(attributes.len(), 9);
        assert!(attributes.contains(&(Key::new("name"), "my-event-name".into())));
        assert!(attributes.contains(&(Key::new("event_id"), 20.into())));
        assert!(attributes.contains(&(Key::new("user_name"), "otel".into())));
        assert!(attributes.contains(&(Key::new("user_email"), "otel@opentelemetry.io".into())));
        #[cfg(feature = "experimental_metadata_attributes")]
        {
            assert!(attributes.contains(&(Key::new("code.filename"), "layer.rs".into())));
            assert!(attributes.contains(&(
                Key::new("code.namespace"),
                "opentelemetry_appender_tracing::layer::tests".into()
            )));
            // The other 3 experimental_metadata_attributes are too unstable to check their value.
            // Ex.: The path will be different on a Windows and Linux machine.
            // Ex.: The line can change easily if someone makes changes in this source file.
            let attributes_key: Vec<Key> = attributes.iter().map(|(key, _)| key.clone()).collect();
            assert!(attributes_key.contains(&Key::new("code.filepath")));
            assert!(attributes_key.contains(&Key::new("code.lineno")));
            assert!(attributes_key.contains(&Key::new("log.target")));
        }
    }

    #[test]
    fn tracing_appender_inside_tracing_context() {
        // Arrange
        let exporter: InMemoryLogsExporter = InMemoryLogsExporter::default();
        let logger_provider = LoggerProvider::builder()
            .with_simple_exporter(exporter.clone())
            .build();

        let layer = layer::OpenTelemetryTracingBridge::new(&logger_provider);
        let subscriber = tracing_subscriber::registry().with(layer);

        // avoiding setting tracing subscriber as global as that does not
        // play well with unit tests.
        let _guard = tracing::subscriber::set_default(subscriber);

        // setup tracing as well.
        let tracer_provider = TracerProvider::builder()
            .with_config(config().with_sampler(Sampler::AlwaysOn))
            .build();
        let tracer = tracer_provider.tracer("test-tracer");

        // Act
        let (trace_id_expected, span_id_expected) = tracer.in_span("test-span", |cx| {
            let trace_id = cx.span().span_context().trace_id();
            let span_id = cx.span().span_context().span_id();

            // logging is done inside span context.
            error!(name: "my-event-name", target: "my-system", event_id = 20, user_name = "otel", user_email = "otel@opentelemetry.io");
            (trace_id, span_id)
        });

        logger_provider.force_flush();

        // Assert TODO: move to helper methods
        let exported_logs = exporter
            .get_emitted_logs()
            .expect("Logs are expected to be exported.");
        assert_eq!(exported_logs.len(), 1);
        let log = exported_logs
            .first()
            .expect("Atleast one log is expected to be present.");

        // validate common fields.
        assert_eq!(log.instrumentation.name, "opentelemetry-appender-tracing");
        assert_eq!(log.record.severity_number, Some(Severity::Error));

        // validate trace context.
        assert!(log.record.trace_context.is_some());
        assert_eq!(
            log.record.trace_context.as_ref().unwrap().trace_id,
            trace_id_expected
        );
        assert_eq!(
            log.record.trace_context.as_ref().unwrap().span_id,
            span_id_expected
        );
        assert_eq!(
            log.record
                .trace_context
                .as_ref()
                .unwrap()
                .trace_flags
                .unwrap(),
            TraceFlags::SAMPLED
        );

        // validate attributes.
        let attributes: Vec<(Key, AnyValue)> = log
            .record
            .attributes
            .clone()
            .expect("Attributes are expected");
        #[cfg(not(feature = "experimental_metadata_attributes"))]
        assert_eq!(attributes.len(), 4);
        #[cfg(feature = "experimental_metadata_attributes")]
        assert_eq!(attributes.len(), 9);
        assert!(attributes.contains(&(Key::new("name"), "my-event-name".into())));
        assert!(attributes.contains(&(Key::new("event_id"), 20.into())));
        assert!(attributes.contains(&(Key::new("user_name"), "otel".into())));
        assert!(attributes.contains(&(Key::new("user_email"), "otel@opentelemetry.io".into())));
        #[cfg(feature = "experimental_metadata_attributes")]
        {
            assert!(attributes.contains(&(Key::new("code.filename"), "layer.rs".into())));
            assert!(attributes.contains(&(
                Key::new("code.namespace"),
                "opentelemetry_appender_tracing::layer::tests".into()
            )));
            // The other 3 experimental_metadata_attributes are too unstable to check their value.
            // Ex.: The path will be different on a Windows and Linux machine.
            // Ex.: The line can change easily if someone makes changes in this source file.
            let attributes_key: Vec<Key> = attributes.iter().map(|(key, _)| key.clone()).collect();
            assert!(attributes_key.contains(&Key::new("code.filepath")));
            assert!(attributes_key.contains(&Key::new("code.lineno")));
            assert!(attributes_key.contains(&Key::new("log.target")));
        }
    }

    #[test]
    fn tracing_appender_standalone_with_tracing_log() {
        // Arrange
        let exporter: InMemoryLogsExporter = InMemoryLogsExporter::default();
        let logger_provider = LoggerProvider::builder()
            .with_simple_exporter(exporter.clone())
            .build();

        let layer = layer::OpenTelemetryTracingBridge::new(&logger_provider);
        let subscriber = tracing_subscriber::registry().with(layer);

        // avoiding setting tracing subscriber as global as that does not
        // play well with unit tests.
        let _guard = tracing::subscriber::set_default(subscriber);
        drop(tracing_log::LogTracer::init());

        // Act
        log::error!("log from log crate");
        logger_provider.force_flush();

        // Assert TODO: move to helper methods
        let exported_logs = exporter
            .get_emitted_logs()
            .expect("Logs are expected to be exported.");
        assert_eq!(exported_logs.len(), 1);
        let log = exported_logs
            .first()
            .expect("Atleast one log is expected to be present.");

        // Validate common fields
        assert_eq!(log.instrumentation.name, "opentelemetry-appender-tracing");
        assert_eq!(log.record.severity_number, Some(Severity::Error));

        // Validate trace context is none.
        assert!(log.record.trace_context.is_none());

        // Validate attributes
        let attributes: Vec<(Key, AnyValue)> = log
            .record
            .attributes
            .clone()
            .expect("Attributes are expected");

        // Attributes can be polluted when we don't use this feature.
        #[cfg(feature = "experimental_metadata_attributes")]
        assert_eq!(attributes.len(), 6);

        assert!(attributes.contains(&(Key::new("name"), "log event".into())));

        #[cfg(feature = "experimental_metadata_attributes")]
        {
            assert!(attributes.contains(&(Key::new("code.filename"), "layer.rs".into())));
            assert!(attributes.contains(&(
                Key::new("code.namespace"),
                "opentelemetry_appender_tracing::layer::tests".into()
            )));
            // The other 3 experimental_metadata_attributes are too unstable to check their value.
            // Ex.: The path will be different on a Windows and Linux machine.
            // Ex.: The line can change easily if someone makes changes in this source file.
            let attributes_key: Vec<Key> = attributes.iter().map(|(key, _)| key.clone()).collect();
            assert!(attributes_key.contains(&Key::new("code.filepath")));
            assert!(attributes_key.contains(&Key::new("code.lineno")));
            assert!(attributes_key.contains(&Key::new("log.target")));
        }
    }

    #[test]
    fn tracing_appender_inside_tracing_context_with_tracing_log() {
        // Arrange
        let exporter: InMemoryLogsExporter = InMemoryLogsExporter::default();
        let logger_provider = LoggerProvider::builder()
            .with_simple_exporter(exporter.clone())
            .build();

        let layer = layer::OpenTelemetryTracingBridge::new(&logger_provider);
        let subscriber = tracing_subscriber::registry().with(layer);

        // avoiding setting tracing subscriber as global as that does not
        // play well with unit tests.
        let _guard = tracing::subscriber::set_default(subscriber);
        drop(tracing_log::LogTracer::init());

        // setup tracing as well.
        let tracer_provider = TracerProvider::builder()
            .with_config(config().with_sampler(Sampler::AlwaysOn))
            .build();
        let tracer = tracer_provider.tracer("test-tracer");

        // Act
        let (trace_id_expected, span_id_expected) = tracer.in_span("test-span", |cx| {
            let trace_id = cx.span().span_context().trace_id();
            let span_id = cx.span().span_context().span_id();

            // logging is done inside span context.
            log::error!("log from log crate");
            (trace_id, span_id)
        });

        logger_provider.force_flush();

        // Assert TODO: move to helper methods
        let exported_logs = exporter
            .get_emitted_logs()
            .expect("Logs are expected to be exported.");
        assert_eq!(exported_logs.len(), 1);
        let log = exported_logs
            .first()
            .expect("Atleast one log is expected to be present.");

        // validate common fields.
        assert_eq!(log.instrumentation.name, "opentelemetry-appender-tracing");
        assert_eq!(log.record.severity_number, Some(Severity::Error));

        // validate trace context.
        assert!(log.record.trace_context.is_some());
        assert_eq!(
            log.record.trace_context.as_ref().unwrap().trace_id,
            trace_id_expected
        );
        assert_eq!(
            log.record.trace_context.as_ref().unwrap().span_id,
            span_id_expected
        );
        assert_eq!(
            log.record
                .trace_context
                .as_ref()
                .unwrap()
                .trace_flags
                .unwrap(),
            TraceFlags::SAMPLED
        );

        // validate attributes.
        let attributes: Vec<(Key, AnyValue)> = log
            .record
            .attributes
            .clone()
            .expect("Attributes are expected");

        // Attributes can be polluted when we don't use this feature.
        #[cfg(feature = "experimental_metadata_attributes")]
        assert_eq!(attributes.len(), 6);

        assert!(attributes.contains(&(Key::new("name"), "log event".into())));

        #[cfg(feature = "experimental_metadata_attributes")]
        {
            assert!(attributes.contains(&(Key::new("code.filename"), "layer.rs".into())));
            assert!(attributes.contains(&(
                Key::new("code.namespace"),
                "opentelemetry_appender_tracing::layer::tests".into()
            )));
            // The other 3 experimental_metadata_attributes are too unstable to check their value.
            // Ex.: The path will be different on a Windows and Linux machine.
            // Ex.: The line can change easily if someone makes changes in this source file.
            let attributes_key: Vec<Key> = attributes.iter().map(|(key, _)| key.clone()).collect();
            assert!(attributes_key.contains(&Key::new("code.filepath")));
            assert!(attributes_key.contains(&Key::new("code.lineno")));
            assert!(attributes_key.contains(&Key::new("log.target")));
        }
    }
}
