use std::env;

pub struct TelemetrySettings {
    pub hostname: String,
    pub attributes: String,
}

impl TelemetrySettings {
    pub fn from_env() -> Self {
        let hostname = env::var_os("OTEL_EXPORTER_OTLP_ENDPOINT")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let attributes: String = env::var_os("OTEL_RESOURCE_ATTRIBUTES")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        Self {
            hostname,
            attributes,
        }
    }
}
