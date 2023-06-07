use api::models::Peer;
use bevy::prelude::Resource;
use client_profile::api_settings::ApiSettings;

#[derive(Resource)]
pub struct Api {
    pub hostname: String,
    pub port: u16,
    pub token: String,
}

impl Api {
    pub fn authenticate(&self) -> Result<Peer, String> {
        api::server_auth(&self.hostname, self.port, &self.token)
    }

    pub fn server_disconnect(&self) -> Result<bool, String> {
        api::server_disconnect(&self.hostname, self.port, &self.token)
    }
}

impl From<&ApiSettings> for Api {
    fn from(value: &ApiSettings) -> Self {
        Api {
            hostname: value.hostname.clone(),
            port: value.port,
            token: value.token.clone(),
        }
    }
}
