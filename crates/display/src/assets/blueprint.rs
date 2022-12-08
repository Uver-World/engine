use bevy::{ecs::system::EntityCommands, prelude::{Image, ImageBundle, default, Transform, Vec3, TextBundle, Commands, Res, AssetServer, NodeBundle, Color, Component, Resource, Handle}, ui::{Style, AlignSelf, PositionType, UiRect, Val, Size}, text::{Text, Font, TextSection, TextStyle}, window::Windows};

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

/*
    Background blueprint
*/
pub fn spawn_blueprint(mut commands: EntityCommands, _assets: &Assets) {
    let obj = Object::new(_assets.clone(), "Blueprint".to_string(), "This is the blueprint".to_string(), false);
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
    obj.spawn(commands);
}

/*
    Spawn box containing the objects to be drags
*/
pub fn spawn_box(mut commands: EntityCommands, _assets: &Assets, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    commands.insert(NodeBundle {
        style: Style {
            position: UiRect::new(Val::Px(0.), Val::Px(0.), Val::Px(0.), Val::Px(0.)),
            position_type: PositionType::Absolute,
            size: Size::new(Val::Px(window.width() * 0.15), Val::Px(window.height())),
            ..default()
        },
        background_color: Color::rgb(1., 1., 1.).into(),
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
