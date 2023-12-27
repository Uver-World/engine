mod frame_time;
mod endpoint;
mod system_usage;

use frame_time::*;
use system_usage::*;

use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::Res;
use opentelemetry::global;

pub use endpoint::TelemetrySettings;


pub struct TelemetryPlugin {
    settings: TelemetrySettings
}

impl TelemetryPlugin {

    pub fn new(settings: TelemetrySettings) -> Self {
        Self {
            settings,
        }
    }

}

impl Plugin for TelemetryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.settings.clone());
        app.insert_resource(FrameTime::new(60)); // Average of FPS over last 60 frames
        app.add_systems(Startup, start)
            .add_systems(Update, (fps_calculation_system, fps_telemetry))
            .add_systems(Update, (cpu_telemetry, ram_telemetry));
    }

}

fn start(settings: Res<TelemetrySettings>) {
    client_telemetry::start_telemetry((&settings).endpoint.clone(), (&settings).token.clone());
}

fn cpu_telemetry() {
    let cpu_usage = get_cpu_usage();
    let meter = global::meter("engine");
    let cpu_gauge = meter.f64_observable_gauge("cpu_usage")
        .with_description("CPU usage percentage")
        .init();
    cpu_gauge.observe(cpu_usage, [].as_ref());
}

fn ram_telemetry() {
    let ram_usage = get_ram_usage();
    let meter = global::meter("engine");
    let ram_gauge = meter.f64_observable_gauge("ram_usage")
        .with_description("RAM usage in megabytes")
        .init();
    ram_gauge.observe(ram_usage, [].as_ref());
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
