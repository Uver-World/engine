use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

#[derive(Component)]
pub struct LoadingBar {
    pub val: f32,
}

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
                width: Val::Percent(50.0),
                height: Val::Percent(4.0),
                align_self: AlignSelf::Center,
                ..default()
            },
            visibility: Visibility::Visible,
            background_color: Color::rgb(91., 91., 91.).into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(0.0),
                        height: Val::Percent(94.0),
                        align_self: AlignSelf::Center,
                        margin: UiRect {
                            left: Val::Px(3.0),
                            right: Val::Px(3.0),
                            top: Val::Px(3.0),
                            bottom: Val::Px(3.0),
                        },
                        ..default()
                    },
                    visibility: Visibility::Visible,
                    background_color: Color::rgb(0., 0., 0.).into(),
                    ..default()
                })
                .insert(LoadingBar { val: 0.0 });
        });
}

pub fn spawn_icon(mut commands: EntityCommands, assets: &Assets) {
    commands.insert(ImageBundle {
        style: Style {
            align_self: AlignSelf::Center,
            ..default()
        },
        transform: Transform::from_scale(Vec3::new(0.5, 0.5, 1.0)),
        image: assets.icon.clone().into(),
        ..default()
    });
}
