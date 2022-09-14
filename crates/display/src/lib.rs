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
        ..Default::default()
    }
}

pub fn run_display() {
    App::new()
        .insert_resource(get_window())
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(cameras::spawn_camera)
        .add_startup_system(assets::loading_screen::load_assets)
        .add_plugin(scenes::loading_screen::LoadingScreenPlugin)
        .add_state(states::DisplayState::LoadingScreen)
        .run()
}
