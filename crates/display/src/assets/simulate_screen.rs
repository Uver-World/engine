use bevy::prelude::Vec3;
use bevy_rapier3d::{na::point, prelude::*, rapier::prelude::ColliderBuilder};
use client_profile::models::{entity::Entity, shape::Shape};

pub fn retrieve_entities(entities: &Vec<Entity>) -> Vec<(Entity, Collider)> {
    let mut shapes = Vec::new();

    for entity in entities.iter() {
        shapes.push((entity.clone(), build_shape(entity)));
    }

    shapes
}

pub fn build_shape(entity: &Entity) -> Collider {
    match entity.group.shape {
        Shape::Rectangle => Collider::cuboid(10.0, 0.1, 10.0),
        Shape::Circle => Collider::cylinder(10.0, 10.0),
        Shape::Triangle => Collider::triangle(
            Vec3::new(-10.0, 0.0, -10.0),
            Vec3::new(10.0, 0.0, -10.0),
            Vec3::new(0.0, 0.0, 10.0),
        ),
    }
}
