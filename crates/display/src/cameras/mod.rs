use bevy::prelude::*;

pub mod camera3d;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
