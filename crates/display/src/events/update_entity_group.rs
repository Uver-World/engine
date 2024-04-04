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
use client_profile::models::{EntityGroup, Location};
use uverworld_packet::update_entity_group::UpdateEntityGroup;

use crate::{
    assets::simulate_screen::{build_shape, shape_to_mesh},
    entities::ui_entity::DisplayEntity,
    scenes::simulate_screen::{spawn_entity, SimulateScreen},
    ClientDisplay,
};

#[derive(Event)]
pub struct UpdateEntityGroupEvent(pub UpdateEntityGroup);

fn update_entities(
    commands: &mut Commands,
    new_group: EntityGroup,
    entities: Vec<(Entity, &DisplayEntity, &Transform)>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    for (entity, display_entity, transform) in entities {
        let mut display_entity = display_entity.clone();
        display_entity.settings.location = Location {
            x: transform.translation.x,
            y: transform.translation.y,
            z: transform.translation.z,
        };
        display_entity.settings.group = new_group.clone();
        display_entity.velocity = Vec3::new(0., 0., 0.);
        commands.entity(entity).despawn_recursive();

        let color = Color::rgb_u8(
            display_entity.settings.group.color.red(),
            display_entity.settings.group.color.green(),
            display_entity.settings.group.color.blue(),
        );
        let collider = build_shape(&display_entity.settings.group.shape);
        let mesh = meshes.add(shape_to_mesh(&display_entity.settings.group.shape));
        let material = materials.add(color.into());
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
}

fn remove_duplicate(target: &str, client: &mut ClientDisplay) {
    client
        .settings
        .profile
        .entity_groups
        .retain_mut(|group| group.name != target);
}

fn handle_update_group_event(
    commands: &mut Commands,
    client: &mut ClientDisplay,
    target: &str,
    new_group: EntityGroup,
    entities: &Query<(Entity, &DisplayEntity, &Transform)>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let entities = entities
        .iter()
        .filter(|(_, display_entity, _)| {
            display_entity.settings.group.name == target
                || display_entity.settings.group.name == new_group.name
        })
        .collect();

    remove_duplicate(&new_group.name, client);

    client
        .settings
        .profile
        .entity_groups
        .push(new_group.clone());

    update_entities(commands, new_group, entities, meshes, materials);
}

pub fn update_entity_group_event(
    mut commands: Commands,
    mut ev: EventReader<UpdateEntityGroupEvent>,
    mut client: ResMut<ClientDisplay>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    entities: Query<(Entity, &DisplayEntity, &Transform)>,
) {
    for event in ev.iter() {
        let new_group = EntityGroup::from_str(&event.0.value).unwrap();
        handle_update_group_event(
            &mut commands,
            &mut client,
            &event.0.target,
            new_group,
            &entities,
            &mut meshes,
            &mut materials,
        );
    }
}
