pub mod exporter_error;

use crate::exporter::exporter_error::SigNozExportError;

use std::fmt::{Debug, Formatter};
use std::sync::mpsc::Sender;
use futures_core::future::BoxFuture;
use opentelemetry::trace::TraceError;
use opentelemetry_sdk::export::trace::{ExportResult, SpanData, SpanExporter};

#[derive(Clone)]
pub struct SigNozExporter {
    span_sender: Sender<Vec<SpanData>>,
}

impl Debug for SigNozExporter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl SigNozExporter {
    pub fn new(span_sender: Sender<Vec<SpanData>>) -> Self {
        Self {
            span_sender
        }
    }

}

impl SpanExporter for SigNozExporter {
    fn export(&mut self, batch: Vec<SpanData>) -> BoxFuture<'static, ExportResult> {
        let span_sender = self.span_sender.clone();
        println!("exporting");
        Box::pin(async move {
            match span_sender.send(batch) {
                Ok(()) => Ok(()),
                Err(err) => Err(TraceError::ExportFailed(Box::new(SigNozExportError::new(&err.to_string())))) // TODO Replace with appropriate error handling
            }
        })
    }
}
