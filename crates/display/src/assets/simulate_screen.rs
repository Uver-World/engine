use bevy::{
    prelude::Vec3,
    render::mesh::Mesh,
};
use bevy_rapier3d::prelude::*;
use bevy::math::primitives::{Cuboid, Cylinder, Sphere};
use client_profile::models::{Entity, Shape};

pub fn retrieve_entities(entities: &Vec<Entity>) -> Vec<(Entity, Collider, Mesh)> {
    let mut shapes = Vec::new();

    for entity in entities.iter() {
        let shape = entity.group.shape.clone();

        shapes.push((entity.clone(), build_shape(&shape), shape_to_mesh(&shape)));
    }

    shapes
}

pub fn build_shape(shape: &Shape) -> Collider {
    match shape {
        Shape::Rectangle => Collider::cuboid(10.0, 10.0, 10.0),
        Shape::Circle => Collider::cylinder(10.0, 10.0),
        Shape::Triangle => Collider::triangle(
            Vec3::new(10.0, 5.0, 10.0),
            Vec3::new(10.0, 5.0, 10.0),
            Vec3::new(10.0, 5.0, 10.0),
        ),
        Shape::Ball => Collider::ball(10.0),
    }
}

pub fn shape_to_mesh(shape: &Shape) -> Mesh {
    match shape {
        Shape::Rectangle => Mesh::from(Cuboid::new(20.0, 20.0, 20.0)),
        Shape::Circle => Mesh::from(Cylinder {
            radius: 10.0,
            half_height: 10.0,
        }),
        Shape::Ball => Mesh::from(Sphere {
            radius: 10.0,
        }),
        _ => todo!("unimplemented!"),
    }
}
