use bevy::{prelude::*, window::WindowDescriptor};

pub mod assets;
pub mod cameras;
pub mod scenes;
pub mod states;

fn get_window() -> WindowDescriptor {
    WindowDescriptor {
        width: 1280.0,
        height: 720.0,
        title: "UverWorld".to_string(),
        ..default()
    }
}

pub fn run_display() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: get_window(),
            ..default()
        }))
        .add_startup_system(cameras::spawn_camera)
        .add_startup_system(assets::blueprint::load_assets)
        .add_plugin(scenes::blueprint::Blueprint)
        .add_state(states::DisplayState::Blueprint)
        .run()
}
