use assets::{blueprint_structure::CursorState, blueprint::Turn};
use bevy::{prelude::*, window::WindowDescriptor};
use bevy_rapier3d::prelude::*;

use assets::blueprint_structure::CursorState;
use client_profile::*;

pub mod assets;
pub mod cameras;
pub mod entities;
pub mod scenes;
pub mod states;

#[derive(Resource)]
pub struct ClientDisplay {
    pub profile: Profile,
    pub is_toggled: bool,
}

impl ClientDisplay {
    fn get_window(&self) -> WindowDescriptor {
        WindowDescriptor {
            width: 1280.0,
            height: 720.0,
            title: "UverWorld".to_string(),
            ..default()
        }
    }

    pub fn run_display(self) {
        App::new()
            .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                window: self.get_window(),
                ..default()
            }))
            // .add_startup_system(cameras::spawn_camera)
            .add_startup_system(assets::loading_screen::load_assets)
            .add_startup_system(assets::blueprint::load_assets)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(assets::blueprint::load_assets)
            .add_plugin(scenes::loading_screen::LoadingScreen)
            .add_plugin(scenes::simulate_screen::SimulateScreen)
            .add_plugin(scenes::blueprint::Blueprint)
            .add_plugin(scenes::blueprint::Blueprint)
            .add_state(states::DisplayState::Blueprint)
            .insert_resource(self)
            .insert_resource(CursorState::default())
            .insert_resource(Turn::default())
            .run()
    }
}
