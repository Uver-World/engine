use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use client_profile::models::entity::Entity;

use super::blueprint::{Assets};

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
    ) -> Self {
        // let transform = Transform {
        //     translation: Vec3::new(0., 0., 0.),
        //     // translation: Vec3::new(get_world_pos(&wnds, &q_camera, pos).x, get_world_pos(&wnds, &q_camera, pos).y, 0.),
        //     scale: Vec3::new(1., 1., 1.),
        //     ..Default::default()
        // };
        // println!("{:?}", transform.translation);
        let bund = ImageBundle {
            style: Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(pos.x),
                    top: Val::Px(pos.y),
                    ..default()
                },
                size: Size {
                    height: Val::Px(size.y),
                    width: Val::Px(size.x),
                },
                ..default()
            },
            // transform: Transform::default(),
            // transform,
            image: asset.icon.clone().into(),
            ..default()
        };
        Self {
            asset: asset.clone().into(),
            name,
            description,
            is_dragable,
            is_pressed,
            pos,
            init_pos: pos,
            bund,
            size,
            is_placed: false,
            obj,
        }
    }

    pub fn spawn(&self, mut commands: EntityCommands) {
        commands
            .insert(self.bund.clone())
            .with_children(|parent| {
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
            })
            .insert(self.clone());
    }

    pub fn clone_at(&self, pos: Vec2) -> Self {
        let mut cloned = self.clone();

        cloned.bund.transform = Transform::from_translation(Vec3::new(pos.x, pos.y, 0.));
        cloned.pos = pos;
        cloned.name = cloned.name + "1";
        (cloned.is_dragable, cloned.is_pressed, cloned.is_placed) = (true, false, false);
        cloned
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
