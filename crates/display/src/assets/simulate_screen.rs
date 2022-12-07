use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::entities::entity::Entity;

pub fn spawn_entity(mut commands: EntityCommands, id: i32) {
    commands
        .insert(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                align_self: AlignSelf::Center,
                ..default()
            },
            visibility: Visibility { is_visible: true },
            background_color: Color::rgb(91., 91., 91.).into(),
            ..default()
        })
        .insert(Entity {
            id: id,
        });
}
