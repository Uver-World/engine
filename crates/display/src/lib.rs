use bevy::{prelude::*, window::WindowResolution, winit::WinitSettings};
use bevy_egui::EguiPlugin;
use bevy_rapier3d::prelude::*;

use client_profile::*;
use filters::Filter;
use states::DisplayState;
use webrtc::{close_matchbox_socket, WebRtc};

use crate::api::Api;

pub mod api;
pub mod assets;
pub mod cameras;
pub mod entities;
pub mod events;
pub mod filters;
pub mod scenes;
pub mod states;
pub mod webrtc;

mod telemetry;

#[derive(Resource)]
pub struct ClientDisplay {
    pub filter: Filter,
    pub settings: Settings,
    pub is_toggled: bool,
    pub tick_rate: f32,
}

impl ClientDisplay {
    fn get_window(&self) -> Window {
        Window {
            resolution: WindowResolution::new(1280.0, 720.0),
            title: "UverWorld".to_string(),
            ..default()
        }
    }

    pub fn run_display(self) {
        // todo, authenticate the server to the api.
        let api = Api::from(&self.settings.api_settings);
        let is_offline = self.settings.is_offline;
        let has_telemetry = self.settings.has_telemetry;
        let telemetry_settings = telemetry::TelemetrySettings::new(
            &self.settings.telemetry_settings.hostname,
            self.settings.telemetry_settings.token.clone(),
        );

        let mut app = App::new();
        app.insert_resource(WinitSettings {
            return_from_run: true,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(self.get_window()),
            ..default()
        }))
        .add_state::<DisplayState>()
        .add_systems(Startup, assets::loading_screen::load_assets)
        .add_plugins(EguiPlugin)
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins((
            scenes::loading_screen::LoadingScreen,
            scenes::simulate_screen::SimulateScreen,
        ));

        if !is_offline {
            app.insert_resource(Api::from(&self.settings.api_settings))
                .add_plugins(WebRtc);
        }

        if has_telemetry {
            app.add_plugins(telemetry::TelemetryPlugin::new(telemetry_settings));
        }

        app.insert_resource(self).run();

        if !is_offline {
            close_matchbox_socket(&api);
        }
    }
}
