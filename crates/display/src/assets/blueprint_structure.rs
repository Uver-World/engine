use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use client_profile::models::entity::Entity;

use super::blueprint::{get_world_pos, Assets};

#[derive(Component)]
pub struct BlueprintBase;

#[derive(Resource, Debug)]
pub struct CursorState {
    pub is_dragging: bool,
    pub is_clicked: bool,
}

#[derive(Component, Clone, Debug)]
pub struct Object {
    pub asset: Assets,
    pub name: String,
    pub description: String,
    pub is_dragable: bool,
    pub is_pressed: bool,
    pub pos: Vec2,
    pub init_pos: Vec2,
    pub bund: ImageBundle,
    pub size: Vec2,
    pub is_placed: bool,
    pub obj: Entity,
}

impl Object {
    pub fn new(
        asset: &Assets,
        name: String,
        description: String,
        is_dragable: bool,
        is_pressed: bool,
        pos: Vec2,
        size: Vec2,
        obj: Entity,
        wnds: Res<Windows>,
        q_camera: Query<(&Camera, &GlobalTransform)>,
    ) -> Self {
        // let world_pos = get_world_pos(&wnds, &q_camera, pos);
        let world_pos = pos;
        let transform = Transform {
            translation: Vec3::new(world_pos.x, world_pos.y, 0.),
            scale: Vec3::new(1., 1., 1.),
            ..Default::default()
        };
        println!("transform init: {:?}", transform);
        Self {
            asset: asset.clone().into(),
            name,
            description,
            is_dragable,
            is_pressed,
            pos,
            init_pos: world_pos,
            bund: ImageBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(world_pos.x),
                        bottom: Val::Px(world_pos.y),
                        ..default()
                    },
                    size: Size {height: Val::Px(size.x), width: Val::Px(size.y) },
                    ..default()
                },
                // transform: Transform::default(),
                transform,
                image: asset.icon.clone().into(),
                ..default()
            },
            size,
            is_placed: false,
            obj,
        }
    }

    pub fn spawn(&self, mut commands: EntityCommands) {
        println!("object = {:?}", self);
        commands.insert(self.bund.clone()).with_children(|parent| {
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
            ]));
        });
    }

    pub fn clone_at(&self, pos: Vec2) -> Self {
        Self {
            asset: self.asset.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            is_dragable: self.is_dragable,
            is_pressed: self.is_pressed,
            pos,
            init_pos: pos,
            bund: ImageBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(pos.x),
                        bottom: Val::Px(pos.y),
                        ..default()
                    },
                    size: Size::new(Val::Px(self.size.x), Val::Px(self.size.y)),
                    ..default()
                },
                // transform: Transform {
                //     translation: Vec3::new(pos.x, pos.y, 0.),
                //     scale: Vec3::new(1., 1., 1.),
                //     ..Default::default()
                // },
                transform: Transform::from_scale(Vec3::new(1., 1., 1.)),
                // transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.)),
                // transform: Transform::from_translation(Vec3::new(world_pos.x, world_pos.y, 0.)),
                image: self.asset.icon.clone().into(),
                ..default()
            },
            size: self.size,
            is_placed: false,
            obj: self.obj.clone(),
        }
    }

    pub fn get_rect(&self) -> UiRect {
        UiRect {
            left: Val::Px(self.pos.x),
            bottom: Val::Px(self.pos.y),
            ..default()
        }
    }
}

impl CursorState {
    pub fn default() -> Self {
        Self {
            is_dragging: false,
            is_clicked: false,
        }
    }
}
