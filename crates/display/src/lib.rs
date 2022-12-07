use bevy::{prelude::*, window::WindowDescriptor};

use client_profile::*;

pub mod assets;
pub mod cameras;
pub mod scenes;
pub mod states;
pub mod entities;

pub struct ClientDisplay {
    pub profile: Profile,
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

    pub fn run_display(&self) {
        App::new()
            .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                window: self.get_window(),
                ..default()
            }))
            .add_startup_system(cameras::spawn_camera)
            .add_startup_system(assets::loading_screen::load_assets)
            .add_plugin(scenes::loading_screen::LoadingScreenPlugin)
            .add_plugin(scenes::simulate_screen::SimulateScreen)
            .add_state(states::DisplayState::LoadingScreen)
            .run()
    }
}
