pub mod models;
pub mod api_settings;
mod profile;

pub use profile::*;

use crate::api_settings::ApiSettings;

pub struct Settings {
    pub profile: Profile,
    pub api_settings: ApiSettings,
    pub is_offline: bool,
    pub is_logging: bool,
}
