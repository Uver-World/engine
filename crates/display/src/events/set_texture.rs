use bevy::{
    asset::{AssetServer, Assets},
    color::Color,
    ecs::{
        entity::Entity,
        event::{Event, EventReader},
        system::{Commands, Query, ResMut},
    },
    hierarchy::DespawnRecursiveExt,
    math::Vec3,
    pbr::StandardMaterial,
    render::mesh::Mesh,
    transform::components::Transform,
};
use bevy_rapier3d::render::ColliderDebugColor;
use client_profile::models::Location;
use uverworld_packet::set_texture::{set_texture::TargetType, SetTexture};

use crate::{
    assets::simulate_screen::{build_shape, shape_to_mesh},
    entities::ui_entity::DisplayEntity,
    scenes::simulate_screen::{spawn_entity, SimulateScreen},
    ClientDisplay,
};

#[derive(Event)]
pub struct SetTextureEvent(pub SetTexture);

fn handle_entity(
    commands: &mut Commands,
    new_texture: &str,
    (entity, display_entity, transform): (Entity, &DisplayEntity, &Transform),
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    assets: &mut ResMut<AssetServer>,
) {
    let mut display_entity = display_entity.clone();
    display_entity.settings.group.texture_id = new_texture.into();
    display_entity.settings.location = Location {
        x: transform.translation.x,
        y: transform.translation.y,
        z: transform.translation.z,
    };
    display_entity.velocity = Vec3::new(0., 0., 0.);
    commands.entity(entity).despawn_recursive();

    let color = Color::srgb_u8(
        display_entity.settings.group.color.red(),
        display_entity.settings.group.color.green(),
        display_entity.settings.group.color.blue(),
    );
    let collider = build_shape(&display_entity.settings.group.shape);
    let mesh = assets.load(new_texture.to_string());
    //assets meshes.add(shape_to_mesh(&display_entity.settings.group.shape));
    let material = materials.add(color);
    spawn_entity(
        commands
            .spawn(SimulateScreen)
            .insert(ColliderDebugColor(color.into())),
        collider,
        mesh,
        material,
        &display_entity,
    );
    println!(
        "applied new_texture: {} to entity_id: {}",
        new_texture, display_entity.id
    )
}

fn handle_entity_group_event(
    commands: &mut Commands,
    _client: &mut ClientDisplay,
    target: &str,
    new_texture: &str,
    entities: &Query<(Entity, &DisplayEntity, &Transform)>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    assets: &mut ResMut<AssetServer>,
) {
    let entities: Vec<_> = entities
        .iter()
        .filter(|(_, display_entity, _)| display_entity.settings.group.name == target)
        .collect();

    for entity in entities {
        handle_entity(commands, new_texture, entity, meshes, materials, assets)
    }
}

fn handle_entity_event(
    commands: &mut Commands,
    _client: &mut ClientDisplay,
    target: &str,
    new_texture: &str,
    entities: &Query<(Entity, &DisplayEntity, &Transform)>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    assets: &mut ResMut<AssetServer>,
) {
    let entity = entities
        .iter()
        .filter(|(_, display_entity, _)| display_entity.id.to_string() == target)
        .nth(0);

    match entity {
        Some(entity) => handle_entity(commands, new_texture, entity, meshes, materials, assets),
        None => println!("entity id: {} could not be found", target),
    }
}

pub fn set_texture_event(
    mut commands: Commands,
    mut ev: EventReader<SetTextureEvent>,
    mut client: ResMut<ClientDisplay>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    entities: Query<(Entity, &DisplayEntity, &Transform)>,
    mut assets: ResMut<AssetServer>,
) {
    for event in ev.read() {
        if event.0.target_type == TargetType::EntityGroup as i32 {
            handle_entity_group_event(
                &mut commands,
                &mut client,
                &event.0.target_id,
                &event.0.texture_id,
                &entities,
                &mut meshes,
                &mut materials,
                &mut assets,
            );
        } else if event.0.target_type == TargetType::Entity as i32 {
            handle_entity_event(
                &mut commands,
                &mut client,
                &event.0.target_id,
                &event.0.texture_id,
                &entities,
                &mut meshes,
                &mut materials,
                &mut assets,
            );
        }
    }
}
