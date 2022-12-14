use bevy::prelude::*;
use bevy_rapier3d::rapier::prelude::*;
use bevy_rapier3d::render::ColliderDebugColor;
use client_profile::models::direction::Direction;
use nalgebra::vector;
use rand::distributions::Uniform;
use rand::prelude::Distribution;

use crate::assets::simulate_screen::retrieve_entities;
use crate::entities::ui_entity::DisplayEntity;
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
                .with_system(update_status)
                .with_system(apply_velocity),
        );
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &mut DisplayEntity)>) {
    for (mut transform, entity) in &mut query {
        transform.translation.x += entity.velocity.x;
        transform.translation.y += entity.velocity.y;
    }
}

fn random_pos(entity: &mut DisplayEntity, transform: &mut Transform) {
    let rand = Uniform::from(1..5).sample(&mut rand::thread_rng()); // TOP BOT, RIGHT, LEFT
    match rand {
        1 => {
            if transform.translation.y > -100.0 {
                entity.velocity.y = -entity.settings.group.speed;
            }
        }
        2 => {
            if transform.translation.y <= 100.0 {
                entity.velocity.y = entity.settings.group.speed;
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

fn update_status(mut query: Query<(&mut Transform, &mut DisplayEntity)>) {
    for (mut transform, mut entity) in &mut query {
        match entity.settings.group.direction.clone() {
            Direction::Random => {
                random_pos(&mut entity, &mut transform);
            }
            Direction::Location(location) => {
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
    let ground_size = 500.;
    let ground_height = 0.1;

    let mut bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();
    let impulse_joints = ImpulseJointSet::new();
    let multibody_joints = MultibodyJointSet::new();

    let rigid_body = RigidBodyBuilder::fixed().translation(vector![0.0, -ground_height, 0.0]);
    let ground_handle = bodies.insert(rigid_body);
    let collider = ColliderBuilder::cuboid(ground_size, ground_height, ground_size);
    colliders.insert_with_parent(collider, ground_handle, &mut bodies);

    let num_z = 8;
    let num_x = 5;
    let shift_y = ground_height + 0.5;
    let shift_z = (num_z as f32 + 2.0) * 2.0;

    let collider = ColliderBuilder::ball(1.0).density(10.0);
    let rigid_body = RigidBodyBuilder::dynamic()
        .linvel(vector![1000.0, 0.0, 0.0])
        .translation(vector![-20.0, shift_y + 2.0, shift_z])
        .ccd_enabled(true);
    let handle = bodies.insert(rigid_body);
    colliders.insert_with_parent(collider.clone(), handle, &mut bodies);
    //    testbed.set_initial_body_color(handle, [0.2, 0.2, 1.0]);

    let mut node = commands.spawn(SimulateScreen);

    node.insert(Camera3dBundle {
        transform: Transform::from_xyz(0., 500.0, 0.).looking_at(Vec3::ZERO, Vec3::X),
        ..Default::default()
    });

    for (entity, shape) in entities {
        commands
            .spawn(shape)
            .insert(DisplayEntity::from_entity(entity.clone(), id))
            .insert(ColliderDebugColor(Color::rgb_u8(
                entity.group.color.red(),
                entity.group.color.green(),
                entity.group.color.blue(),
            )))
            .insert(TransformBundle::from(Transform::from_xyz(
                entity.location.x,
                entity.location.y,
                0.0,
            )));
        id += 1;
    }
}

fn destroy(mut commands: Commands, query: Query<Entity, With<SimulateScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
