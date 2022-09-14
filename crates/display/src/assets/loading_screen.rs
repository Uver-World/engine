use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

#[derive(Component)]
pub struct LoadingBar;

pub struct Assets {
    pub font: Handle<Font>,
    pub icon: Handle<Image>,
}

pub fn load_assets(mut commands: Commands, assets: Res<AssetServer>) {
    let ui_assets = Assets {
        font: assets.load("FiraCode-Regular.ttf"),
        icon: assets.load("uverworld_icon.png"),
    };

    commands.insert_resource(ui_assets);
}

pub fn spawn_icon(mut commands: EntityCommands, assets: &Assets) {}
