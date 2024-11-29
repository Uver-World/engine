use bevy::{
    asset::Assets,
    color::Color,
    ecs::{
        entity::Entity,
        event::{Event, EventReader},
        system::{Commands, Query, ResMut},
    },
    hierarchy::DespawnRecursiveExt,
    pbr::StandardMaterial,
    render::mesh::Mesh,
    transform::components::Transform,
};

use bevy_rapier3d::render::ColliderDebugColor;
use client_profile::models::{self, Direction, EntityGroup, Location, Range, SightRadius};
use uverworld_packet::update_entities::{direction, EntityBatch};

use crate::{
    assets::simulate_screen::{build_shape, shape_to_mesh},
    entities::ui_entity::DisplayEntity,
    scenes::simulate_screen::{spawn_entity, SimulateScreen},
    ClientDisplay,
};

#[derive(Event)]
pub struct UpdateEntitiesEvent(pub EntityBatch);

fn set_entities(
    commands: &mut Commands,
    new_entities: Vec<DisplayEntity>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    for new_entity in new_entities {
        let color = Color::srgb_u8(
            new_entity.settings.group.color.red(),
            new_entity.settings.group.color.green(),
            new_entity.settings.group.color.blue(),
        );
        let collider = build_shape(&new_entity.settings.group.shape);
        let mesh = meshes.add(shape_to_mesh(&new_entity.settings.group.shape));
        let material = materials.add(color);
        spawn_entity(
            commands
                .spawn(SimulateScreen)
                .insert(ColliderDebugColor(color.into())),
            collider,
            mesh,
            material,
            &new_entity,
        );
    }
}

fn convert_directions(
    raw_directions: Vec<uverworld_packet::update_entities::Direction>,
) -> Vec<Direction> {
    let mut directions = Vec::new();

    for direction in raw_directions {
        let direction = match direction.direction.unwrap() {
            direction::Direction::Random(random) => {
                let range = random.location.unwrap();
                Direction::Random(Range {
                    x: [-range.x, range.x],
                    y: [-range.y, range.y],
                    z: [-range.z, range.z],
                })
            }
            direction::Direction::Location(position) => {
                let location = Location {
                    x: position.x,
                    y: position.y,
                    z: position.z,
                };
                Direction::Location(location)
            }
            direction::Direction::Static(_) => Direction::Static,
            direction::Direction::Follow(follow) => {
                let sight_radius = SightRadius(follow.sight_radius);
                Direction::Follow(sight_radius, follow.entity_group_names)
            }
            direction::Direction::Escape(escape) => {
                let sight_radius = SightRadius(escape.sight_radius);
                Direction::Escape(sight_radius, escape.entity_group_names)
            }
        };
        directions.push(direction);
    }

    directions
}

fn convert_group(new_raw_group: uverworld_packet::update_entities::EntityGroup) -> EntityGroup {
    let new_group = EntityGroup {
        name: new_raw_group.name,
        color: models::Color::Custom(new_raw_group.color),
        speed: new_raw_group.speed,
        directions: convert_directions(new_raw_group.direction),
        shape: models::Shape::Rectangle,
        // shape: models::Shape::from_str(&new_raw_group.shape),
        gravity: new_raw_group.gravity,
        texture_id: new_raw_group.texture_id,
    };

    println!("new group found: {:#?}", new_group);

    new_group
}

fn convert_groups(
    new_raw_groups: Vec<uverworld_packet::update_entities::EntityGroup>,
) -> Vec<EntityGroup> {
    let mut new_groups = Vec::new();

    for new_raw_group in new_raw_groups {
        new_groups.push(convert_group(new_raw_group));
    }

    new_groups
}

fn convert_entity(
    id: usize,
    entity_group: &EntityGroup,
    raw_entity: uverworld_packet::update_entities::Entities,
) -> DisplayEntity {
    let location = if let Some(position) = raw_entity.location {
        Location::new(position.x, position.y, position.z)
    } else {
        Location::new(100., 100., 100.)
    };
    let new_entity = client_profile::models::Entity::new(entity_group.clone(), location);

    DisplayEntity::from_entity(new_entity, id)
}

fn convert_entities(
    client: &ClientDisplay,
    new_raw_entities: Vec<uverworld_packet::update_entities::Entities>,
) -> Vec<DisplayEntity> {
    let mut new_entities = Vec::new();
    let mut id = 0;

    for new_raw_entity in new_raw_entities {
        let entity_group = client
            .settings
            .profile
            .get_entity_group_by_name(&new_raw_entity.group);
        match entity_group {
            Some(entity_group) => {
                println!("entity with group name: [{}] found", entity_group.name);
                let new_entity = convert_entity(id, entity_group, new_raw_entity);
                new_entities.push(new_entity);
                id += 1;
            }
            None => eprintln!(
                "Cannot find entity_group name: [{:?}]",
                new_raw_entity.group
            ),
        }
    }

    new_entities
}

fn set_groups(client: &mut ClientDisplay, new_groups: Vec<EntityGroup>) {
    client.settings.profile.entity_groups.clear();

    for entity_group in new_groups {
        client.settings.profile.entity_groups.push(entity_group);
    }
}

fn handle_update_entities_event(
    commands: &mut Commands,
    client: &mut ClientDisplay,
    entity_batch: EntityBatch,
    entities: &Query<(Entity, &DisplayEntity, &Transform)>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    for (entity, _, _) in entities {
        commands.entity(entity).despawn_recursive();
    }

    set_groups(client, convert_groups(entity_batch.entity_groups));
    println!(
        "groups set! size: {}",
        client.settings.profile.entity_groups.len()
    );

    set_entities(
        commands,
        convert_entities(client, entity_batch.entities),
        meshes,
        materials,
    );
}

pub fn update_entities_event(
    mut commands: Commands,
    mut ev: EventReader<UpdateEntitiesEvent>,
    mut client: ResMut<ClientDisplay>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    entities: Query<(Entity, &DisplayEntity, &Transform)>,
) {
    for event in ev.read() {
        println!("{:?}", &event.0.entities);
        println!("{:?}", &event.0.entity_groups);
        handle_update_entities_event(
            &mut commands,
            &mut client,
            event.0.clone(), // TODO remove clone and take value instead
            &entities,
            &mut meshes,
            &mut materials,
        );
    }
}
