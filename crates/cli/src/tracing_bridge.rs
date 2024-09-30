/// This file is a modified version of the opentelemetry-appender-tracing crate.
/// We need to use this to bridge between the opentelemetry and the tracing crate.
/// The opentelemetry-appender-tracing crate has version mismatch issues.
use opentelemetry::{
    logs::{AnyValue, LogRecord, Logger, LoggerProvider, Severity},
    Key,
};
use std::borrow::Cow;
use tracing_core::Level;
#[cfg(feature = "experimental_metadata_attributes")]
use tracing_core::Metadata;
#[cfg(feature = "experimental_metadata_attributes")]
use tracing_log::NormalizeEvent;
use tracing_subscriber::Layer;

const INSTRUMENTATION_LIBRARY_NAME: &str = "opentelemetry-appender-tracing";

/// Visitor to record the fields from the event record.
struct EventVisitor<'a, LR: LogRecord> {
    log_record: &'a mut LR,
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

impl<'a, LR: LogRecord> EventVisitor<'a, LR> {
    fn new(log_record: &'a mut LR) -> Self {
        EventVisitor { log_record }
    }

    #[cfg(feature = "experimental_metadata_attributes")]
    fn visit_experimental_metadata(&mut self, meta: &Metadata) {
        if let Some(module_path) = meta.module_path() {
            self.log_record.add_attribute(
                Key::new("code.namespace"),
                AnyValue::from(module_path.to_owned()),
            );
        }

        if let Some(filepath) = meta.file() {
            self.log_record.add_attribute(
                Key::new("code.filepath"),
                AnyValue::from(filepath.to_owned()),
            );
            self.log_record.add_attribute(
                Key::new("code.filename"),
                AnyValue::from(get_filename(filepath).to_owned()),
            );
        }

        if let Some(line) = meta.line() {
            self.log_record
                .add_attribute(Key::new("code.lineno"), AnyValue::from(line));
        }
    }
}

impl<'a, LR: LogRecord> tracing::field::Visit for EventVisitor<'a, LR> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        #[cfg(feature = "experimental_metadata_attributes")]
        if is_duplicated_metadata(field.name()) {
            return;
        }
        if field.name() == "message" {
            self.log_record.set_body(format!("{:?}", value).into());
        } else {
            self.log_record
                .add_attribute(Key::new(field.name()), AnyValue::from(format!("{value:?}")));
        }
    }

    fn record_str(&mut self, field: &tracing_core::Field, value: &str) {
        #[cfg(feature = "experimental_metadata_attributes")]
        if is_duplicated_metadata(field.name()) {
            return;
        }
        //TODO: Consider special casing "message" to populate body and document
        // to users to use message field for log message, to avoid going to the
        // record_debug, which has dyn dispatch, string allocation and
        // formatting cost.

        //TODO: Fix heap allocation. Check if lifetime of &str can be used
        // to optimize sync exporter scenario.
        self.log_record
            .add_attribute(Key::new(field.name()), AnyValue::from(value.to_owned()));
    }

    fn record_bool(&mut self, field: &tracing_core::Field, value: bool) {
        self.log_record
            .add_attribute(Key::new(field.name()), AnyValue::from(value));
    }

    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        self.log_record
            .add_attribute(Key::new(field.name()), AnyValue::from(value));
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        #[cfg(feature = "experimental_metadata_attributes")]
        if is_duplicated_metadata(field.name()) {
            return;
        }
        self.log_record
            .add_attribute(Key::new(field.name()), AnyValue::from(value));
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
            logger: provider
                .logger_builder(INSTRUMENTATION_LIBRARY_NAME)
                .with_version(Cow::Borrowed(env!("CARGO_PKG_VERSION")))
                .build(),
            _phantom: Default::default(),
        }
    }
}

impl<S, P, L> Layer<S> for OpenTelemetryTracingBridge<P, L>
where
    S: tracing::Subscriber,
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

        let mut log_record = self.logger.create_log_record();

        // TODO: Fix heap allocation
        log_record.set_target(meta.target().to_string());
        log_record.set_event_name(meta.name());
        log_record.set_severity_number(severity_of_level(meta.level()));
        log_record.set_severity_text(meta.level().as_str());
        let mut visitor = EventVisitor::new(&mut log_record);
        #[cfg(feature = "experimental_metadata_attributes")]
        visitor.visit_experimental_metadata(meta);
        // Visit fields.
        event.record(&mut visitor);

        //emit record
        self.logger.emit(log_record);
    }

    #[cfg(feature = "logs_level_enabled")]
    fn event_enabled(
        &self,
        _event: &tracing_core::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        let severity = severity_of_level(_event.metadata().level());
        self.logger
            .event_enabled(severity, _event.metadata().target())
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
    use opentelemetry_sdk::logs::{LogRecord, LoggerProvider};
    use opentelemetry_sdk::testing::logs::InMemoryLogsExporter;
    use opentelemetry_sdk::trace;
    use opentelemetry_sdk::trace::{Sampler, TracerProvider};
    use tracing::error;
    use tracing_subscriber::layer::SubscriberExt;

    pub fn attributes_contains(log_record: &LogRecord, key: &Key, value: &AnyValue) -> bool {
        log_record
            .attributes_iter()
            .any(|(k, v)| k == key && v == value)
    }

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
        #[cfg(not(feature = "experimental_metadata_attributes"))]
        assert_eq!(log.record.attributes_iter().count(), 3);
        #[cfg(feature = "experimental_metadata_attributes")]
        assert_eq!(log.record.attributes_iter().count(), 7);
        assert!(attributes_contains(
            &log.record,
            &Key::new("event_id"),
            &AnyValue::Int(20)
        ));
        assert!(attributes_contains(
            &log.record,
            &Key::new("user_name"),
            &AnyValue::String("otel".into())
        ));
        assert!(attributes_contains(
            &log.record,
            &Key::new("user_email"),
            &AnyValue::String("otel@opentelemetry.io".into())
        ));
        #[cfg(feature = "experimental_metadata_attributes")]
        {
            assert!(attributes_contains(
                &log.record,
                &Key::new("code.filename"),
                &AnyValue::String("layer.rs".into())
            ));
            assert!(attributes_contains(
                &log.record,
                &Key::new("code.namespace"),
                &AnyValue::String("opentelemetry_appender_tracing::layer::tests".into())
            ));
            // The other 3 experimental_metadata_attributes are too unstable to check their value.
            // Ex.: The path will be different on a Windows and Linux machine.
            // Ex.: The line can change easily if someone makes changes in this source file.
            let attributes_key: Vec<Key> = log
                .record
                .attributes_iter()
                .map(|(key, _)| key.clone())
                .collect();
            assert!(attributes_key.contains(&Key::new("code.filepath")));
            assert!(attributes_key.contains(&Key::new("code.lineno")));
            assert!(!attributes_key.contains(&Key::new("log.target")));
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
            .with_config(trace::Config::default().with_sampler(Sampler::AlwaysOn))
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
        #[cfg(not(feature = "experimental_metadata_attributes"))]
        assert_eq!(log.record.attributes_iter().count(), 3);
        #[cfg(feature = "experimental_metadata_attributes")]
        assert_eq!(log.record.attributes_iter().count(), 7);
        assert!(attributes_contains(
            &log.record,
            &Key::new("event_id"),
            &AnyValue::Int(20.into())
        ));
        assert!(attributes_contains(
            &log.record,
            &Key::new("user_name"),
            &AnyValue::String("otel".into())
        ));
        assert!(attributes_contains(
            &log.record,
            &Key::new("user_email"),
            &AnyValue::String("otel@opentelemetry.io".into())
        ));
        #[cfg(feature = "experimental_metadata_attributes")]
        {
            assert!(attributes_contains(
                &log.record,
                &Key::new("code.filename"),
                &AnyValue::String("layer.rs".into())
            ));
            assert!(attributes_contains(
                &log.record,
                &Key::new("code.namespace"),
                &AnyValue::String("opentelemetry_appender_tracing::layer::tests".into())
            ));
            // The other 3 experimental_metadata_attributes are too unstable to check their value.
            // Ex.: The path will be different on a Windows and Linux machine.
            // Ex.: The line can change easily if someone makes changes in this source file.
            let attributes_key: Vec<Key> = log
                .record
                .attributes_iter()
                .map(|(key, _)| key.clone())
                .collect();
            assert!(attributes_key.contains(&Key::new("code.filepath")));
            assert!(attributes_key.contains(&Key::new("code.lineno")));
            assert!(!attributes_key.contains(&Key::new("log.target")));
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

        // Attributes can be polluted when we don't use this feature.
        #[cfg(feature = "experimental_metadata_attributes")]
        assert_eq!(log.record.attributes_iter().count(), 4);

        #[cfg(feature = "experimental_metadata_attributes")]
        {
            assert!(attributes_contains(
                &log.record,
                &Key::new("code.filename"),
                &AnyValue::String("layer.rs".into())
            ));
            assert!(attributes_contains(
                &log.record,
                &Key::new("code.namespace"),
                &AnyValue::String("opentelemetry_appender_tracing::layer::tests".into())
            ));
            // The other 3 experimental_metadata_attributes are too unstable to check their value.
            // Ex.: The path will be different on a Windows and Linux machine.
            // Ex.: The line can change easily if someone makes changes in this source file.
            let attributes_key: Vec<Key> = log
                .record
                .attributes_iter()
                .map(|(key, _)| key.clone())
                .collect();
            assert!(attributes_key.contains(&Key::new("code.filepath")));
            assert!(attributes_key.contains(&Key::new("code.lineno")));
            assert!(!attributes_key.contains(&Key::new("log.target")));
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
            .with_config(trace::Config::default().with_sampler(Sampler::AlwaysOn))
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

        // Attributes can be polluted when we don't use this feature.
        #[cfg(feature = "experimental_metadata_attributes")]
        assert_eq!(log.record.attributes_iter().count(), 4);

        #[cfg(feature = "experimental_metadata_attributes")]
        {
            assert!(attributes_contains(
                &log.record,
                &Key::new("code.filename"),
                &AnyValue::String("layer.rs".into())
            ));
            assert!(attributes_contains(
                &log.record,
                &Key::new("code.namespace"),
                &AnyValue::String("opentelemetry_appender_tracing::layer::tests".into())
            ));
            // The other 3 experimental_metadata_attributes are too unstable to check their value.
            // Ex.: The path will be different on a Windows and Linux machine.
            // Ex.: The line can change easily if someone makes changes in this source file.
            let attributes_key: Vec<Key> = log
                .record
                .attributes_iter()
                .map(|(key, _)| key.clone())
                .collect();
            assert!(attributes_key.contains(&Key::new("code.filepath")));
            assert!(attributes_key.contains(&Key::new("code.lineno")));
            assert!(!attributes_key.contains(&Key::new("log.target")));
        }
    }
}
