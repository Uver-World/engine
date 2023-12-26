mod trace_worker;

pub use trace_worker::TraceWorker;

pub struct TelemetryWorker {
    span_worker: TraceWorker,
}

impl TelemetryWorker {

    pub fn new(span_worker: TraceWorker) -> Self {
        Self {
            span_worker
        }
    }

    pub fn launch(self) {
        self.span_worker.launch();
    }

}