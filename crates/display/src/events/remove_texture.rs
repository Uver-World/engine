use std::fs;

use bevy::{
    asset::Assets,
    ecs::{
        entity::Entity,
        event::{Event, EventReader},
        system::{Commands, Query, ResMut},
    },
    hierarchy::DespawnRecursiveExt,
    math::Vec3,
    pbr::StandardMaterial,
    render::{color::Color, mesh::Mesh},
    transform::components::Transform,
};
use bevy_rapier3d::render::ColliderDebugColor;
use client_profile::models::Location;
use uverworld_packet::remove_texture::RemoveTexture;

use crate::{
    assets::simulate_screen::{build_shape, shape_to_mesh},
    entities::ui_entity::DisplayEntity,
    scenes::simulate_screen::{spawn_entity, SimulateScreen},
    ClientDisplay,
};

#[derive(Event)]
pub struct RemoveTextureEvent(pub RemoveTexture);

fn handle_entity(
    commands: &mut Commands,
    (entity, display_entity, transform): (Entity, &DisplayEntity, &Transform),
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let mut display_entity = display_entity.clone();
    display_entity.settings.group.texture_id = String::new();
    display_entity.settings.location = Location {
        x: transform.translation.x,
        y: transform.translation.y,
        z: transform.translation.z,
    };
    display_entity.velocity = Vec3::new(0., 0., 0.);
    commands.entity(entity).despawn_recursive();

    let color = Color::rgb_u8(
        display_entity.settings.group.color.red(),
        display_entity.settings.group.color.green(),
        display_entity.settings.group.color.blue(),
    );
    let collider = build_shape(&display_entity.settings.group.shape);
    let mesh = meshes.add(shape_to_mesh(&display_entity.settings.group.shape));
    let material = materials.add(color);
    spawn_entity(
        commands
            .spawn(SimulateScreen)
            .insert(ColliderDebugColor(color)),
        collider,
        mesh,
        material,
        &display_entity,
    );
}

fn handle_remove_event(
    commands: &mut Commands,
    _client: &mut ClientDisplay,
    removed_texture_id: &str,
    entities: &Query<(Entity, &DisplayEntity, &Transform)>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let entities: Vec<_> = entities
        .iter()
        .filter(|(_, display_entity, _)| {
            display_entity.settings.group.texture_id == removed_texture_id
        })
        .collect();

    for entity in entities {
        handle_entity(commands, entity, meshes, materials);
        println!(
            "removed texture: {} to entity_id: {}",
            removed_texture_id, entity.1.id
        );
    }
}

pub fn remove_texture_event(
    mut commands: Commands,
    mut ev: EventReader<RemoveTextureEvent>,
    mut client: ResMut<ClientDisplay>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    entities: Query<(Entity, &DisplayEntity, &Transform)>,
) {
    for event in ev.read() {
        match fs::remove_file(format!("textures/{}", event.0.texture_id)) {
            Ok(_) => {
                println!("texture {} file removed!", event.0.texture_id);
                handle_remove_event(
                    &mut commands,
                    &mut client,
                    &event.0.texture_id,
                    &entities,
                    &mut meshes,
                    &mut materials,
                );
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
