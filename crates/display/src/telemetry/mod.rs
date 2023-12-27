mod frame_time;
mod endpoint;

pub use frame_time::*;

use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::Res;
use opentelemetry::global;

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
            .add_systems(Update, (fps_calculation_system, fps_telemetry));
    }

}

fn start(endpoint: Res<TelemetryEndpoint>) {
    client_telemetry::start_telemetry((&endpoint).0.clone())
}

fn fps_telemetry(frame_time: Res<FrameTime>) {
    let fps = frame_time.average_fps();

    //let tracer = global::tracer("engine");
    let meter = global::meter("engine");

    //let mut span = tracer.start("frame_info");
    //span.set_attribute(KeyValue::new("fps", fps));

    let histogram = meter.f64_observable_gauge("fps")
        .with_description("Frames per second")
        .init();

    // Include the span context as a label in the histogram observation
    histogram.observe(fps, [].as_ref());

    //span.end();
}
