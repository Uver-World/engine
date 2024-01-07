use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RigidBody};
use bevy_rapier3d::render::ColliderDebugColor;
use client_profile::models::direction::Direction;
use client_profile::models::color::Color as ClientColor;
use client_profile::models::location::Location;
use rand::distributions::Uniform;
use rand::prelude::Distribution;

use crate::assets::simulate_screen::retrieve_entities;
use crate::cameras::camera3d::{Camera3D, Camera3DPlugin};
use crate::entities::ui_entity::DisplayEntity;
use crate::states::DisplayState;
use crate::ClientDisplay;

use opentelemetry::global;

#[derive(Component)]
pub struct SimulateScreen;

impl Plugin for SimulateScreen {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(DisplayState::SimulateScreen), construct)
            .add_systems(Update, (update_status, apply_velocity, keyboard_input).run_if(in_state(DisplayState::SimulateScreen)))
            .add_systems(OnExit(DisplayState::SimulateScreen), destroy)
            .add_plugins(Camera3DPlugin);
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &mut DisplayEntity)>) {
    for (mut transform, entity) in &mut query {
        transform.translation.x += entity.velocity.x;
        transform.translation.y += entity.velocity.y;
        transform.translation.z += entity.velocity.z;
    }
}

fn random_pos(entity: &mut DisplayEntity, transform: &mut Transform) {
    let rand = Uniform::from(1..5).sample(&mut rand::thread_rng()); // TOP BOT, RIGHT, LEFT
    match rand {
        1 => {
            if transform.translation.z > -100.0 {
                entity.velocity.z = -entity.settings.group.speed;
            }
        }
        2 => {
            if transform.translation.z <= 100.0 {
                entity.velocity.z = entity.settings.group.speed;
            }
        }
        3 => {
            if transform.translation.x <= 300.0 {
                entity.velocity.x = entity.settings.group.speed;
            }
        }
        _ => {
            if transform.translation.x > 20.0 {
                entity.velocity.x = -entity.settings.group.speed;
            }
        }
    }
}

fn destination_pos(entity: &mut DisplayEntity, transform: &Transform, location: Location) {
    if transform.translation.x < location.x {
        entity.velocity.x = entity.settings.group.speed;
    }
    if transform.translation.x > location.x {
        entity.velocity.x = -entity.settings.group.speed;
    }
    if transform.translation.y < location.y {
        entity.velocity.y = entity.settings.group.speed;
    }
    if transform.translation.y > location.y {
        entity.velocity.y = -entity.settings.group.speed;
    }
    if transform.translation.z < location.z {
        entity.velocity.z = entity.settings.group.speed;
    }
    if transform.translation.z > location.z {
        entity.velocity.z = -entity.settings.group.speed;
    }
    if transform.translation.x >= location.x - 10. && transform.translation.x <= location.x + 10. {
        entity.velocity.x = 0.;
    }
    if transform.translation.y >= location.y - 10. && transform.translation.y <= location.y + 10. {
        entity.velocity.y = 0.;
    }
    if transform.translation.z >= location.z - 10. && transform.translation.z <= location.z + 10. {
        entity.velocity.z = 0.;
    }
}

fn follow_pos(
    target: &mut DisplayEntity,
    transform: &Transform,
    group_target: Vec<String>,
    query: &Vec<(DisplayEntity, Transform)>,
) {
    let mut location: Option<Location> = None;
    for (entity, ent_transform) in query {
        // We check if the group is not the same, or target != entity
        if !group_target.contains(&entity.settings.group.group) || target == entity {
            continue;
        }
        match location {
            Some(found_location) => {
                if found_location.x + found_location.y + found_location.z
                    > ent_transform.translation.x
                        + ent_transform.translation.y
                        + ent_transform.translation.z
                {
                    location = Some(Location::new(
                        ent_transform.translation.x,
                        ent_transform.translation.y,
                        ent_transform.translation.z,
                    ));
                }
            }
            None => {
                location = Some(Location::new(
                    ent_transform.translation.x,
                    ent_transform.translation.y,
                    ent_transform.translation.z,
                ))
            }
        }
    }
    match location {
        Some(location) => destination_pos(target, transform, location),
        _ => {}
    }
}

fn escape_pos(
    target: &mut DisplayEntity,
    transform: &Transform,
    group_target: Vec<String>,
    query: &Vec<(DisplayEntity, Transform)>,
) {
    let mut location: Option<Location> = None;
    for (entity, ent_transform) in query {
        // We check if the group is not the same, or target != entity
        if !group_target.contains(&entity.settings.group.group) || target == entity {
            continue;
        }

        match location {
            Some(found_location) => {
                if found_location.x + found_location.y + found_location.z
                    > ent_transform.translation.x
                        + ent_transform.translation.y
                        + ent_transform.translation.z
                {
                    location = Some(Location::new(
                        ent_transform.translation.x,
                        ent_transform.translation.y,
                        ent_transform.translation.z,
                    ));
                }
            }
            None => {
                location = Some(Location::new(
                    transform.translation.x,
                    transform.translation.y,
                    transform.translation.z,
                ))
            }
        }
    }
    match location {
        Some(location) => {
            let length = Location::new(
                transform.translation.x - location.x,
                transform.translation.y - location.y,
                transform.translation.z - location.z,
            );
            let (x, y, z) = if length.x < 0.0 {
                if length.y < 0.0 {
                    (
                        transform.translation.x + length.x,
                        transform.translation.y + length.y,
                        transform.translation.z + length.z,
                    )
                } else {
                    (
                        transform.translation.x + length.x,
                        transform.translation.y - length.y,
                        transform.translation.z + length.z,
                    )
                }
            } else {
                if length.y >= 0.0 {
                    (
                        transform.translation.x - length.x,
                        transform.translation.y - length.y,
                        transform.translation.z + length.z,
                    )
                } else {
                    (
                        transform.translation.x - length.x,
                        transform.translation.y + length.y,
                        transform.translation.z + length.z,
                    )
                }
            };
            destination_pos(target, transform, Location::new(x, y, z));
        }
        _ => {}
    }
}

fn update_status(mut query: Query<(&mut DisplayEntity, &mut Transform)>) {
    let entities: Vec<(DisplayEntity, Transform)> = query
        .iter()
        .map(|(entity, transform)| (entity.clone(), transform.clone()))
        .collect();

    for (mut ui_entity, mut transform) in &mut query {
        for direction in ui_entity.settings.group.directions.clone() {
            match direction {
                Direction::Random => random_pos(&mut ui_entity, &mut transform),
                Direction::Location(location) => {
                    destination_pos(&mut ui_entity, &transform, location)
                }
                Direction::Follow(group_name) => {
                    follow_pos(&mut ui_entity, &transform, group_name, &entities)
                }
                Direction::Escape(group_name) => {
                    escape_pos(&mut ui_entity, &transform, group_name, &entities)
                }
                Direction::Static => {}
            }
        }
    }
}

fn construct(mut commands: Commands, client: Res<ClientDisplay>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let entities = retrieve_entities(client.settings.profile.get_entities());

    // TODO switch other place the number of entities recorded.
    let meter = global::meter("engine");
    let ram_gauge = meter.u64_observable_gauge("entities")
        .with_description("Number of entities")
        .init();
    ram_gauge.observe(entities.len() as u64, [].as_ref());
    
    let mut id = 0;

    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(20., 500.0, 20.).looking_at(Vec3::ZERO, Vec3::X),
            ..Default::default()
        })
        .insert(Camera3D {
            x: 300.,
            distance: 300.,
            center: Vec3::new(0., 50., 0.),
            rotate_sensitivity: 0.05,
            ..Camera3D::default()
        });

    commands
        .spawn_empty()
        .insert(Collider::cuboid(1000.0, -0.1, 1000.0))
        .insert(ColliderDebugColor(Color::rgb_u8(0, 255, 0)))
        .insert(TransformBundle::from(Transform::from_xyz(0., -2., 0.)));

    commands
        .spawn_empty()
        .insert(Collider::cuboid(1000.0, -0.1, 1000.0))
        .insert(ColliderDebugColor(Color::rgb_u8(0, 255, 0)))
        .insert(TransformBundle::from(Transform::from_xyz(0., 2000., 0.)));

    commands
        .spawn_empty()
        .insert(Collider::cuboid(1000.0, 1000.0, -0.1))
        .insert(ColliderDebugColor(Color::rgb_u8(255, 0, 0)))
        .insert(TransformBundle::from(Transform::from_xyz(0., 998., 1000.)));

    commands
        .spawn_empty()
        .insert(Collider::cuboid(1000.0, 1000.0, -0.1))
        .insert(ColliderDebugColor(Color::rgb_u8(255, 0, 0)))
        .insert(TransformBundle::from(Transform::from_xyz(0., 998., -1000.)));

    commands
        .spawn_empty()
        .insert(Collider::cuboid(-0.1, 1000.0, 1000.0))
        .insert(ColliderDebugColor(Color::rgb_u8(0, 0, 255)))
        .insert(TransformBundle::from(Transform::from_xyz(-1000., 998., 0.)));

    commands
        .spawn_empty()
        .insert(Collider::cuboid(-0.1, 1000.0, 1000.0))
        .insert(ColliderDebugColor(Color::rgb_u8(0, 0, 255)))
        .insert(TransformBundle::from(Transform::from_xyz(1000., 998., 0.)));

    for (entity, collider, mesh) in entities {
        let color = Color::rgb_u8(
                entity.group.color.red(),
                entity.group.color.green(),
                entity.group.color.blue(),
            );

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(color.into()), ..Default::default()})

            .insert(collider)

            .insert(RigidBody::Dynamic)
            .insert(DisplayEntity::from_entity(entity.clone(), id))
            .insert(ColliderDebugColor(color))
            .insert(TransformBundle::from(Transform::from_xyz(
                entity.location.x,
                entity.location.y,
                entity.location.z,
            )));
        id += 1;
    }
}

fn destroy(mut commands: Commands, query: Query<Entity, With<SimulateScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut client: ResMut<ClientDisplay>,
    entities: Query<(&DisplayEntity, &mut Visibility)>
) {
    if keys.just_pressed(KeyCode::B) {
        client.filter.toggle_color_filter(ClientColor::Red, entities);
    }
}
