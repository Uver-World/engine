use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

#[derive(Component)]
pub struct LoadingBar;

#[derive(Resource)]
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

pub fn spawn_loading_bar(mut commands: EntityCommands, _assets: &Assets) {
    commands
        .insert(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(50.0), Val::Percent(4.0)),
                align_self: AlignSelf::Center,
                ..default()
            },
            visibility: Visibility { is_visible: true },
            background_color: Color::rgb(91., 91., 91.).into(),
            ..default()
        })
        .insert(LoadingBar);
}

pub fn spawn_icon(mut commands: EntityCommands, assets: &Assets) {
    commands.insert(ImageBundle {
        style: Style {
            align_self: AlignSelf::Center,
            ..default()
        },
        image: assets.icon.clone().into(),
        ..default()
    });
}
