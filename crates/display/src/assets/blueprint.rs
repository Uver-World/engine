use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

#[derive(Component)]
pub struct BlueprintBase;

#[derive(Resource)]
pub struct Assets {
    pub font: Handle<Font>,
    pub icon: Handle<Image>,
}

#[derive(Component)]
struct Object {
    pub asset: &Assets,
    pub name: String,
    pub description: String,
    pub is_dragable: bool,
}

impl Object {
    pub fn new(asset: &Assets, name: String, description: String, is_dragable: bool) -> Self {
        Self {
            asset,
            name,
            description,
            is_dragable,
        }
    }

    pub fn spawn(&self, mut commands: EntityCommands) {
        commands.insert(ImageBundle {
            style: Style {
                align_self: AlignSelf::Center,
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(1., 1., 1.)),
            image: self.asset.icon.clone().into(),
            ..default()
        });
        self.spawn_text(commands, self.asset);
    }

    pub fn spawn_text(&self, mut commands: EntityCommands, assets: &Assets) {
        commands.insert((
            TextBundle::from_sections([
                TextSection::new(
                    self.name.clone(),
                    TextStyle {
                        font: self.asset.font.clone(),
                        font_size: 60.0,
                        color: Color::BLACK,
                    },
                ),
            ]),
            TextBundle::from_sections([
                TextSection::new(
                    self.description.clone(),
                    TextStyle {
                        font: self.asset.font.clone(),
                        font_size: 60.0,
                        color: Color::GREEN,
                    },
                ),
            ]),
        ));
    }
}

pub fn load_assets(mut commands: Commands, assets: Res<AssetServer>) {
    let ui_assets = Assets {
        font: assets.load("FiraCode-Regular.ttf"),
        icon: assets.load("Blueprint.png"),
    };

    commands.insert_resource(ui_assets);
}

pub fn spawn_blueprint(mut commands: EntityCommands, _assets: &Assets) {
    commands.insert(ImageBundle {
        style: Style {
            align_self: AlignSelf::Center,
            position_type: PositionType::Absolute,
            ..default()
        },
        transform: Transform::from_scale(Vec3::new(2.5, 2.5, 2.5)),
        image: _assets.icon.clone().into(),
        ..default()
    });
}

pub fn spawn_box(mut commands: EntityCommands, _assets: &Assets, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    commands.insert(NodeBundle {
        // sprite: Sprite {
        //     color: Color::rgb(0.25, 0.25, 0.75),
        //     custom_size: Some(Vec2::new(50.0, 100.0)),
        //     ..default()
        // },
        style: Style {
            position: UiRect::new(Val::Px(0.), Val::Px(0.), Val::Px(0.), Val::Px(0.)),
            position_type: PositionType::Absolute,
            size: Size::new(Val::Px(window.width() * 0.15), Val::Px(window.height())),
            ..default()
        },
        background_color: Color::rgba(1., 1., 1., 0.6).into(),
        ..default()
    });
}

// pub fn spawn_icon(mut commands: EntityCommands, assets: &Assets) {
//     commands.insert(ImageBundle {
//         style: Style {
//             align_self: AlignSelf::Center,
//             ..default()
//         },
//         transform: Transform::from_scale(Vec3::new(0.5, 0.5, 1.0)),
//         image: assets.icon.clone().into(),
//         ..default()
//     });
// }
