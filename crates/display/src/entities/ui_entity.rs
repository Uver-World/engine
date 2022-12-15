use bevy::prelude::*;
use client_profile::models::entity::Entity;

#[derive(Component, Clone, PartialEq)]
pub struct DisplayEntity {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub settings: Entity,
    pub velocity: Vec3,
}

impl DisplayEntity {
    pub fn get_rect(&self) -> UiRect {
        UiRect {
            left: Val::Px(self.x),
            top: Val::Px(self.y),
            ..default()
        }
    }

    pub fn from_entity(entity: Entity, id: usize) -> Self {
        Self {
            id,
            x: entity.location.x,
            y: entity.location.y,
            settings: entity,
            velocity: Vec3::new(0., 0., 0.),
        }
    }
}
