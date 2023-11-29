pub mod models;
mod profile;
mod api_settings;
mod tracer_settings;

pub use api_settings::ApiSettings;
pub use tracer_settings::TracerSettings;

pub use profile::*;

pub struct Settings {
    pub profile: Profile,
    pub api_settings: ApiSettings,
    pub tracer_settings: TracerSettings,
    pub is_offline: bool,
    pub is_logging: bool,
}
