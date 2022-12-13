use bevy::prelude::*;
use client_profile::models::entity::Entity;

#[derive(Component)]
pub struct UiEntity {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub settings: Entity,
}

impl UiEntity {
    pub fn get_rect(&self) -> UiRect {
        UiRect {
            left: Val::Px(self.x),
            top: Val::Px(self.y),
            ..default()
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            id: self.id,
            x: self.x,
            y: self.y,
            settings: self.settings.clone(),
        }
    }
}
