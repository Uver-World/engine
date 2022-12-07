use bevy::prelude::*;

#[derive(Component)]
pub struct Entity {
    pub id: i32,
}

impl Default for Entity {
    fn default() -> Self {
        Entity {
            id: 0,
        }
    }
}
