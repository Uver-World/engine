use bevy::prelude::*;
use client_profile::models::direction::Direction;
use client_profile::models::location::{self, Location};
use rand::distributions::Uniform;
use rand::prelude::Distribution;

use crate::assets::simulate_screen::retrieve_entities;
use crate::entities::ui_entity::UiEntity;
use crate::states::DisplayState;
use crate::ClientDisplay;

#[derive(Component)]
pub struct SimulateScreen;

impl Plugin for SimulateScreen {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(DisplayState::SimulateScreen).with_system(construct),
        )
        .add_system_set(SystemSet::on_exit(DisplayState::SimulateScreen).with_system(destroy))
        .add_system_set(
            SystemSet::on_update(DisplayState::SimulateScreen).with_system(update_status),
        );
    }
}

fn random_pos(ui_entity: &mut UiEntity) {
    let rand = Uniform::from(1..5).sample(&mut rand::thread_rng()); // TOP BOT, RIGHT, LEFT
    match rand {
        1 => {
            if ui_entity.y > -100.0 {
                ui_entity.y -= ui_entity.settings.group.speed;
            }
        }
        2 => {
            if ui_entity.y <= 100.0 {
                ui_entity.y += ui_entity.settings.group.speed;
            }
        }
        3 => {
            if ui_entity.x <= 300.0 {
                ui_entity.x += ui_entity.settings.group.speed;
            }
        }
        _ => {
            if ui_entity.x > 20.0 {
                ui_entity.x -= ui_entity.settings.group.speed;
            }
        }
    }
}

fn destination_pos(ui_entity: &mut UiEntity, location: Location) {
    if ui_entity.x < location.x {
        ui_entity.x += ui_entity.settings.group.speed;
    }
    if ui_entity.x > location.x {
        ui_entity.x -= ui_entity.settings.group.speed;
    }
    if ui_entity.y < location.y {
        ui_entity.y += ui_entity.settings.group.speed;
    }
    if ui_entity.y > location.y {
        ui_entity.y -= ui_entity.settings.group.speed;
    }
}

fn follow_pos(target: &mut UiEntity, group_target: String, query: &Vec<UiEntity>) {
    let mut location: Option<Location> = None;
    for entity in query {
        // We check if the group is not the same, or target != entity
        if group_target != entity.settings.group.group || target == entity {
            continue;
        }

        match location {
            Some(found_location) => {
                if found_location.x + found_location.y > entity.x + entity.y {
                    location = Some(Location::new(entity.x, entity.y));
                }
            }
            None => location = Some(Location::new(entity.x, entity.y)),
        }
    }
    match location {
        Some(location) => destination_pos(target, location),
        _ => {}
    }
}

fn escape_pos(target: &mut UiEntity, group_target: String, query: &Vec<UiEntity>) {
    let mut location: Option<Location> = None;
    for entity in query {
        // We check if the group is not the same, or target != entity
        if group_target != entity.settings.group.group || target == entity {
            continue;
        }

        match location {
            Some(found_location) => {
                if found_location.x + found_location.y > entity.x + entity.y {
                    location = Some(Location::new(entity.x, entity.y));
                }
            }
            None => location = Some(Location::new(entity.x, entity.y)),
        }
    }
    match location {
        Some(location) => {
            let length = Location::new(target.x - location.x, target.y - location.y);

            if length.x < 0.0 {
                if length.y < 0.0 {
                    destination_pos(
                        target,
                        Location::new(target.x + length.x, target.y + length.y),
                    );
                } else {
                    destination_pos(
                        target,
                        Location::new(target.x + length.x, target.y - length.y),
                    );
                }
            } else {
                if length.y >= 0.0 {
                    destination_pos(
                        target,
                        Location::new(target.x - length.x, target.y - length.y),
                    );
                } else {
                    destination_pos(
                        target,
                        Location::new(target.x - length.x, target.y + length.y),
                    );
                }
            }
        }
        _ => {}
    }
}

fn update_status(mut query: Query<(&mut Transform, &mut UiEntity)>) {
    let entities: Vec<UiEntity> = query.iter().map(|(_, entity)| entity.clone()).collect();

    for (mut style, mut ui_entity) in &mut query {
        match ui_entity.settings.group.direction.clone() {
            Direction::Random => {
                random_pos(&mut ui_entity);
            }
            Direction::Location(location) => {
                destination_pos(&mut ui_entity, location);
            }
            Direction::Follow(group_name) => follow_pos(&mut ui_entity, group_name, &entities),
            Direction::Escape(group_name) => escape_pos(&mut ui_entity, group_name, &entities),
        }
        style.translation = Vec3::new(ui_entity.x, ui_entity.y, 1.);
    }
}

fn construct(mut commands: Commands, client: Res<ClientDisplay>) {
    let entities = retrieve_entities(client.profile.get_entities());
    let mut id = 0;

    for (entity, shape) in entities {
        commands
            .spawn(shape)
            .insert(UiEntity::from_entity(entity, id));
        id += 1;
    }
}

fn destroy(mut commands: Commands, query: Query<Entity, With<SimulateScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
