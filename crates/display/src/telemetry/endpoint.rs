use bevy::prelude::Resource;

#[derive(Clone, Resource)]
pub struct TelemetrySettings {
    pub endpoint: String,
    pub token: Option<String>,
}

impl TelemetrySettings {

    pub fn new<T: Into<String>>(endpoint: T, token: Option<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            token
        }
    }

}
