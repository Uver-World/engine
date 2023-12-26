use bevy::prelude::Resource;

#[derive(Clone, Resource)]
pub struct TelemetryEndpoint(pub String);

impl TelemetryEndpoint {

    pub fn new<T: Into<String>>(endpoint: T) -> Self {
        Self(endpoint.into())
    }

}
