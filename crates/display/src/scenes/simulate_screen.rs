use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use client_profile::models::direction::Direction;
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
            SystemSet::on_update(DisplayState::SimulateScreen)
                .with_system(check_collision)
                .with_system(update_status.before(check_collision))
                .with_system(apply_velocity.before(check_collision)),
        );
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &mut UiEntity)>) {
    for (mut transform, entity) in &mut query {
        transform.translation.x += entity.velocity.x;
        transform.translation.y += entity.velocity.y;
    }
}

fn check_collision(mut query: Query<(&mut Transform, &mut UiEntity)>) {
    let entities: Vec<(Transform, UiEntity)> = query
        .iter()
        .map(|(transform, entity)| (transform.clone(), entity.clone()))
        .collect();

    for (transform1, mut entity1) in &mut query {
        for (transform2, entity2) in &entities {
            if entity1.id == entity2.id {
                continue;
            }

            let collision = collide(
                transform1.translation,
                Vec2::new(120., 120.),
                transform2.translation,
                Vec2::new(120., 120.),
            );

            if let Some(collision) = collision {
                if collision == Collision::Left {
                    entity1.velocity.x -= entity1.settings.group.speed;
                    // entity1.x -= entity1.settings.group.speed;
                } else if collision == Collision::Right {
                    entity1.velocity.x += entity1.settings.group.speed;
                    // entity1.x += entity1.settings.group.speed;
                }

                if collision == Collision::Top {
                    entity1.velocity.y += entity1.settings.group.speed;
                    // entity1.y += entity1.settings.group.speed;
                } else if collision == Collision::Bottom {
                    entity1.velocity.y -= entity1.settings.group.speed;
                    // entity1.y -= entity1.settings.group.speed;
                }

                // transform1.translation = Vec3::new(entity1.x, entity1.y, 1.);
            }
        }
    }
}

fn random_pos(entity: &mut UiEntity, transform: &mut Transform) {
    let rand = Uniform::from(1..5).sample(&mut rand::thread_rng()); // TOP BOT, RIGHT, LEFT
    match rand {
        1 => {
            if transform.translation.y > -100.0 {
                entity.velocity.y -= entity.settings.group.speed;
            }
        }
        2 => {
            if transform.translation.y <= 100.0 {
                entity.velocity.y += entity.settings.group.speed;
            }
        }
        3 => {
            if transform.translation.x <= 300.0 {
                entity.velocity.x += entity.settings.group.speed;
            }
        }
        _ => {
            if transform.translation.x > 20.0 {
                entity.velocity.x -= entity.settings.group.speed;
            }
        }
    }
}

fn update_status(mut query: Query<(&mut Transform, &mut UiEntity)>) {
    for (mut transform, mut entity) in &mut query {
        match entity.settings.group.direction.clone() {
            Direction::Random => {
                random_pos(&mut entity, &mut transform);
            }
            Direction::Location(location) => {
                if transform.translation.x < location.x {
                    entity.velocity.x += entity.settings.group.speed;
                }
                if transform.translation.x > location.x {
                    entity.velocity.x -= entity.settings.group.speed;
                }
                if transform.translation.y < location.y {
                    entity.velocity.y += entity.settings.group.speed;
                }
                if transform.translation.y > location.y {
                    entity.velocity.y -= entity.settings.group.speed;
                }
                if transform.translation.x >= location.x - 10.
                    && transform.translation.x <= location.x + 10.
                {
                    entity.velocity.x = 0.;
                }
                if transform.translation.y >= location.y - 10.
                    && transform.translation.y <= location.y + 10.
                {
                    entity.velocity.y = 0.;
                }
            }
        }
        // style.translation = Vec3::new(ui_entity.x, ui_entity.y, 1.);
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
