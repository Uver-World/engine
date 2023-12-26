use std::sync::mpsc;
use opentelemetry::{global, KeyValue};
use opentelemetry::global::GlobalTracerProvider;

use opentelemetry_sdk::export::trace::{SpanExporter};
use opentelemetry_sdk::{Resource, trace as sdktrace};
use opentelemetry_sdk::trace::Config;

use crate::exporter::SigNozExporter;
use crate::worker::TelemetryWorker;

pub fn start_telemetry(endpoint: String)  {
    // Create the channel
    let (span_sender, span_receiver) = mpsc::channel();

    // Initialize the exporter
    let span_exporter = SigNozExporter::new(span_sender);

    // Initialize other telemetry components
    let _ = init_telemetry(span_exporter);

    // Start the telemetry worker thread
    TelemetryWorker::new(span_receiver, endpoint).launch();
}

fn init_telemetry(span_exporter: impl SpanExporter + 'static) -> GlobalTracerProvider {
    let provider = sdktrace::TracerProvider::builder()
        .with_simple_exporter(span_exporter).with_config(
        Config::default().with_resource(
            Resource::new(vec![
                KeyValue::new("service.name", "engine")
            ])
        ))
        .build();

    global::set_tracer_provider(provider)
}
