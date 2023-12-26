mod frame_time;
mod endpoint;

pub use frame_time::*;

use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::Res;
use opentelemetry::{global, KeyValue};
use opentelemetry::trace::{Span, Tracer};

pub use endpoint::TelemetryEndpoint;


pub struct TelemetryPlugin {
    endpoint: TelemetryEndpoint
}

impl TelemetryPlugin {

    pub fn new(endpoint: TelemetryEndpoint) -> Self {
        Self {
            endpoint
        }
    }

}

impl Plugin for TelemetryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.endpoint.clone());
        app.insert_resource(FrameTime::new(60)); // Average of FPS over last 60 frames
        app.add_systems(Startup, start)
            .add_systems(Update, (refresh, fps_calculation_system));
    }

}

fn start(endpoint: Res<TelemetryEndpoint>) {
    client_telemetry::start_telemetry((&endpoint).0.clone())
}

pub fn refresh(frame_time: Res<FrameTime>) {
    fps_telemetry(frame_time.average_fps());
}

fn fps_telemetry(fps: f64) {
    let tracer = global::tracer("engine");
    let meter = global::meter("engine");

    let mut span = tracer.start("frame_info");
    span.set_attribute(KeyValue::new("fps", fps));

    // Extract the span context as a string or another suitable format
    let span_context = format!("{}", span.span_context().trace_id());

    let histogram = meter.f64_observable_counter("fps")
        .with_description("Frames per second")
        .init();

    // Include the span context as a label in the histogram observation
    //histogram.observe(fps, [KeyValue::new("span_context", span_context)].as_ref());
    histogram.observe(fps, [].as_ref());

    span.end();



}
