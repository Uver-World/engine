use bevy::{prelude::*, window::WindowResolution, winit::WinitSettings};
use bevy_rapier3d::prelude::*;

use client_profile::{api_settings::ApiSettings, *};
use matchbox_socket::close_matchbox_socket;
use states::DisplayState;

use crate::api::Api;

pub mod api;
pub mod assets;
pub mod cameras;
pub mod entities;
pub mod matchbox_socket;
pub mod scenes;
pub mod states;

#[derive(Resource)]
pub struct ClientDisplay {
    pub profile: Profile,
    pub is_toggled: bool,
    pub api_settings: ApiSettings,
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
        let api = Api::from(&self.api_settings);

        App::new()
            .insert_resource(WinitSettings {
                return_from_run: true,
                ..default()
            })
            .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .insert_resource(Api::from(&self.api_settings))
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(self.get_window()),
                ..default()
            }))
            .add_state::<DisplayState>()
            .add_startup_system(assets::loading_screen::load_assets)
            .add_startup_system(matchbox_socket::start_matchbox_socket)
            .add_system(matchbox_socket::check_peers)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(scenes::loading_screen::LoadingScreen)
            .add_plugin(scenes::simulate_screen::SimulateScreen)
            .insert_resource(self)
            .run();
        close_matchbox_socket(&api);
    }
}
