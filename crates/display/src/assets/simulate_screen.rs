use bevy::math::Vec3;
use bevy::prelude::{Color, Transform};
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use client_profile::models::{entity::Entity, shape::Shape};

pub fn retrieve_entities(entities: &Vec<Entity>) -> Vec<(Entity, ShapeBundle)> {
    let mut shapes = Vec::new();

    for entity in entities.iter() {
        shapes.push((entity.clone(), build_shape(entity)));
    }

    shapes
}

pub fn build_shape(entity: &Entity) -> ShapeBundle {
    let transform: Transform = Transform {
        translation: (Vec3::new(entity.location.x, entity.location.y, 0.)),
        ..Transform::default()
    };
    match entity.group.shape {
        Shape::Circle => GeometryBuilder::build_as(
            &shapes::Circle {
                radius: 60.0,
                ..shapes::Circle::default()
            },
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::rgb_u8(
                    entity.group.color.red(),
                    entity.group.color.green(),
                    entity.group.color.blue(),
                )),
                outline_mode: StrokeMode::new(Color::BLACK, 10.0),
            },
            transform,
        ),
        Shape::Rectangle => GeometryBuilder::build_as(
            &shapes::RegularPolygon {
                sides: 4,
                feature: shapes::RegularPolygonFeature::Radius(80.0),
                ..shapes::RegularPolygon::default()
            },
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::rgb_u8(
                    entity.group.color.red(),
                    entity.group.color.green(),
                    entity.group.color.blue(),
                )),
                outline_mode: StrokeMode::new(Color::BLACK, 10.0),
            },
            transform,
        ),
        Shape::Triangle => GeometryBuilder::build_as(
            &shapes::RegularPolygon {
                sides: 3,
                feature: shapes::RegularPolygonFeature::Radius(80.0),
                ..shapes::RegularPolygon::default()
            },
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::rgb_u8(
                    entity.group.color.red(),
                    entity.group.color.green(),
                    entity.group.color.blue(),
                )),
                outline_mode: StrokeMode::new(Color::BLACK, 10.0),
            },
            transform,
        ),
        /*_ => GeometryBuilder::build_as(
            &shapes::RegularPolygon {
                sides: 6,
                feature: shapes::RegularPolygonFeature::Radius(80.0),
                ..shapes::RegularPolygon::default()
            },
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::rgb_u8(
                    entity.group.color.red(),
                    entity.group.color.green(),
                    entity.group.color.blue(),
                )),
                outline_mode: StrokeMode::new(Color::BLACK, 10.0),
            },
            Transform::default(),
        ),*/
    }
}
