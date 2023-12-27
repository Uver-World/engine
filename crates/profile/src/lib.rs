pub mod models;
mod profile;
mod api_settings;
mod telemetry_settings;

pub use api_settings::ApiSettings;
pub use telemetry_settings::TelemetrySettings;

pub use profile::*;

pub struct Settings {
    pub profile: Profile,
    pub api_settings: ApiSettings,
    pub telemetry_settings: TelemetrySettings,
    pub is_offline: bool,
    pub has_telemetry: bool,
}
