use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use client_profile::models::entity::Entity;

use crate::entities::ui_entity::UiEntity;

pub fn spawn_entities(mut commands: EntityCommands, entities: &Vec<Entity>) {
    for (id, entity) in entities.iter().enumerate() {
        commands.with_children(|parent| {
            spawn_entity(parent.spawn_empty(), entity, id);
        });
    }
}

pub fn spawn_entity(mut commands: EntityCommands, entity: &Entity, id: usize) {
    let ui_entity = UiEntity {
        x: entity.location.x,
        y: entity.location.y,
        id,
        settings: entity.clone(),
    };

    commands
        .insert(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                position: ui_entity.get_rect(),
                align_self: AlignSelf::Center,
                ..default()
            },
            visibility: Visibility { is_visible: true },
            background_color: Color::rgb(
                entity.group.color.red(),
                entity.group.color.green(),
                entity.group.color.blue(),
            )
            .into(),
            ..default()
        })
        .insert(ui_entity);
}
