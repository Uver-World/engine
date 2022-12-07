use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use client_profile::models::entity::Entity;

use crate::entities::entity;

pub fn spawn_entities(mut commands: EntityCommands, entities: &Vec<Entity>) {
    for entity in entities {
        commands.with_children(|parent| {
            spawn_entity(parent.spawn_empty(), entity);
        });
    }
}

pub fn spawn_entity(mut commands: EntityCommands, entity: &Entity) {
    commands
        .insert(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                align_self: AlignSelf::Center,
                ..default()
            },
            visibility: Visibility { is_visible: true },
            background_color: Color::rgb(
                entity.group.color.red(),
                entity.group.color.blue(),
                entity.group.color.green(),
            )
            .into(),
            ..default()
        })
        .insert(entity::Entity {
            left: entity.location.x,
            top: entity.location.y,
            ..default()
        });
}
