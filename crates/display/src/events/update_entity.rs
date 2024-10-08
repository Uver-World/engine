use bevy::{
    asset::Assets,
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
use uverworld_packet::update_entity::{
    self, update_entity::UpdateType, UpdateEntity, UpdateGroup, UpdatePosition,
};

use crate::{
    assets::simulate_screen::{build_shape, shape_to_mesh},
    entities::ui_entity::DisplayEntity,
    scenes::simulate_screen::{spawn_entity, SimulateScreen},
    ClientDisplay,
};

#[derive(Event)]
pub struct UpdateEntityEvent(pub UpdateEntity);

fn handle_update_group_event(
    commands: &mut Commands,
    client: &mut ClientDisplay,
    target: u64,
    update_group: UpdateGroup,
    entities: &Query<(Entity, &DisplayEntity, &mut Transform)>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let (entity, display_entity, transform) = entities
        .iter()
        .filter(|(_, display_entity, _)| display_entity.id == target as usize)
        .nth(0)
        .unwrap();
    let new_group = client
        .settings
        .profile
        .entity_groups
        .iter()
        .filter(|group| group.name == update_group.new_group)
        .nth(0)
        .unwrap();

    let mut display_entity = display_entity.clone();
    display_entity.settings.location = Location {
        x: transform.translation.x,
        y: transform.translation.y,
        z: transform.translation.z,
    };
    display_entity.settings.group = new_group.clone();
    display_entity.velocity = Vec3::new(0., 0., 0.);
    commands.entity(entity).despawn_recursive();

    let color = Color::srgb_u8(
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
            .insert(ColliderDebugColor(color.into())),
        collider,
        mesh,
        material,
        &display_entity,
    );
}

fn handle_update_position_event(
    target: u64,
    update_position: UpdatePosition,
    entities: &mut Query<(Entity, &DisplayEntity, &mut Transform)>,
) {
    let (_, _, mut transform) = entities
        .iter_mut()
        .filter(|(_, display_entity, _)| display_entity.id == target as usize)
        .nth(0)
        .unwrap();
    transform.translation.x = update_position.new_x;
    transform.translation.y = update_position.new_y;
    transform.translation.z = update_position.new_z;
}

pub fn update_entity_event(
    mut commands: Commands,
    mut ev: EventReader<UpdateEntityEvent>,
    mut client: ResMut<ClientDisplay>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut entities: Query<(Entity, &DisplayEntity, &mut Transform)>,
) {
    for event in ev.read() {
        if event.0.r#type == UpdateType::Group as i32 {
            let new_group = update_entity::deserialize_update_group(&event.0.value).unwrap();
            handle_update_group_event(
                &mut commands,
                &mut client,
                event.0.target,
                new_group,
                &entities,
                &mut meshes,
                &mut materials,
            );
        } else if event.0.r#type == UpdateType::Position as i32 {
            let new_position = update_entity::deserialize_update_position(&event.0.value).unwrap();
            handle_update_position_event(event.0.target, new_position, &mut entities)
        }
    }
}
