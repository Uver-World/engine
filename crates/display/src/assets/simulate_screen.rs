use bevy::{
    prelude::Vec3,
    render::mesh::{shape, Mesh},
};
use bevy_rapier3d::prelude::*;
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
        Shape::Rectangle => Mesh::from(shape::Box::new(20.0, 20.0, 20.0)),
        Shape::Circle => Mesh::from(shape::Cylinder {
            height: 20.0,
            radius: 10.0,
            resolution: 10,
            segments: 10,
        }),
        Shape::Ball => Mesh::from(shape::UVSphere {
            radius: 10.0,
            sectors: 32,
            stacks: 16,
        }),
        _ => todo!("unimplemented!"),
    }
}
