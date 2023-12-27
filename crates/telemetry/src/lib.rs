mod provider;
mod worker;

use crate::provider::{SigNozMeter, SigNozTracer};
use crate::worker::TelemetryWorker;

pub fn start_telemetry(endpoint: String, token: Option<String>)  {
    let trace_worker = SigNozTracer::setup(endpoint.clone());
    let meter_worker = SigNozMeter::setup(endpoint.clone(), token.clone());

    TelemetryWorker::new(trace_worker, meter_worker).launch();
}
