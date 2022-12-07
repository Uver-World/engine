use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Entity {
    pub id: i32,
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Entity {
    pub fn get_rect(&self) -> UiRect {
        UiRect { left: Val::Px(self.left), right: Val::Px(self.right), top: Val::Px(self.top), bottom: Val::Px(self.bottom) }
    }
}
