use bevy::ecs::entity;
use bevy::{ecs::system::EntityCommands};
use bevy::prelude::*;

#[derive(Component)]
pub struct BlueprintBase;

#[derive(Resource, Clone)]
pub struct Assets {
    pub font: Handle<Font>,
    pub icon: Handle<Image>,
}

#[derive(Component)]
pub struct Object {
    pub asset: Assets,
    pub name: String,
    pub description: String,
    pub is_dragable: bool,
    pub is_pressed: bool,
}

impl Object {
    pub fn new(asset: &Assets, name: String, description: String, is_dragable: bool, is_pressed: bool) -> Self {
        Self {
            asset: asset.clone().into(),
            name,
            description,
            is_dragable,
            is_pressed,
        }
    }

    pub fn spawn(&self, mut commands: EntityCommands) {
        commands.insert(ButtonBundle  {
            style: Style {
                align_self: AlignSelf::Center,
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(1., 1., 1.)),
            image: self.asset.icon.clone().into(),
            ..default()
        }).with_children(|parent| {
            parent.spawn(TextBundle::from_sections([
                TextSection::new(
                    self.name.clone(),
                    TextStyle {
                        font: self.asset.font.clone(),
                        font_size: 20.0,
                        color: Color::RED,
                    },
                ),
                TextSection::new(
                    self.description.clone(),
                    TextStyle {
                        font: self.asset.font.clone(),
                        font_size: 15.0,
                        color: Color::BLUE,
                    },
                ),
                ])
            );
        });
    }
}

pub fn drag(mut commands: Commands, buttons: Res<Input<MouseButton>>, mut query: Query<(Entity, With<Object>, &mut Object)>) {
    for (mut _entity, _, mut _object) in &mut query {
        if buttons.pressed(MouseButton::Left) {
            if _object.is_dragable {
                println!("Drag");
                commands.get_entity(_entity);
            }
        } else if _object.is_pressed {
            println!("Release");
            _object.is_pressed = false;
        }
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
    let obj = Object::new(_assets, "Button 1".to_string(), "First button".to_string(), true, false);
    let obj2 = Object::new(_assets, "Button 2".to_string(), "Second button".to_string(), true, false);
    commands.with_children(|parent| obj.spawn(parent.spawn_empty()));
    commands.with_children(|parent| obj2.spawn(parent.spawn_empty()));
    commands.insert(obj);
    commands.insert(obj2);
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
