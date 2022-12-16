use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use client_profile::models::direction::Direction;
use client_profile::models::shape::Shape;

use crate::ClientDisplay;
use client_profile::models::entity::{Entity, EntityGroup};
use client_profile::models::location::Location;

use super::blueprint_structure::*;

#[derive(Resource, Clone, Debug)]
pub struct Assets {
    pub font: Handle<Font>,
    pub icon: Handle<Image>,
}

pub fn get_world_pos(
    wnds: &Res<Windows>,
    q_camera: &Query<(&Camera, &GlobalTransform)>,
    pos: Vec2,
) -> Vec2 {
    let (camera, camera_transform) = q_camera.single();
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };
    let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
    let ndc = (pos / window_size) * 2.0 - Vec2::ONE;
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
    let world_pos: Vec2 = world_pos.truncate();
    return world_pos;
}

pub fn is_in_rect(obj: Object, pos: Vec2) -> bool {
    println!(
        "pos: {:?}; point: {:?}; size: {:?}; final pos: {:?}",
        obj.pos,
        pos,
        obj.size,
        obj.pos + obj.size
    );
    pos.x >= obj.pos.x
        && pos.x <= obj.pos.x + obj.size.x
        && pos.y >= obj.pos.y
        && pos.y <= obj.pos.y + obj.size.y
}

#[derive(Resource, Debug)]
pub struct Turn {
    pub is_turn: f32,
}

impl Turn {
    pub fn default() -> Self {
        Self { is_turn: 0. }
    }
}

pub fn drag(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut client: ResMut<ClientDisplay>,
    mut query: Query<(
        bevy::prelude::Entity,
        With<Object>,
        &mut Object,
        &mut Style,
        &mut Transform,
    )>,
    mut cursor_state: ResMut<CursorState>,
    mut turn: ResMut<Turn>,
) {
    for (_entity, _, mut object, mut style, mut transform) in &mut query {
        let mut is_good = false;
        let mut screen_pos = Vec2::new(0., 0.);
        if buttons.pressed(MouseButton::Left) {
            turn.is_turn += 1.;
            let wnd = windows.get_primary().unwrap();
            if let Some(screen_posb) = wnd.cursor_position() {
                if is_in_rect(object.clone(), screen_posb) || object.is_pressed {
                    is_good = true;
                    screen_pos = screen_posb;
                } else {
                    if !cursor_state.is_dragging {
                        continue;
                    }
                }
            }
            cursor_state.is_clicked = if cursor_state.is_clicked && !object.is_pressed {
                println!("cursor {:?} {:?}", cursor_state, turn);
                continue;
            } else {
                true
            };
            println!("drag: {:?}", object);
            if object.is_dragable {
                println!("dragable: {}", object.name);
                if is_good {
                    cursor_state.is_dragging = true;
                    object.pos = Vec2::new(screen_pos.x - object.size.x / 2., screen_pos.y);
                }
                style.position = object.get_rect();
                transform.translation = object.pos.extend(0.);
                object.is_pressed = true;
            }
        } else if cursor_state.is_clicked {
            if !cursor_state.is_dragging {
                println!("click without drag");
                cursor_state.is_clicked = false;
            } else if object.is_pressed {
                println!("clone");
                (
                    cursor_state.is_clicked,
                    cursor_state.is_dragging,
                    object.is_dragable,
                    object.is_pressed,
                ) = (false, false, false, false);
                let cpy = object.clone_at(object.init_pos);
                let mut empty = commands.spawn_empty();
                cpy.spawn(&mut empty);
                client.profile.add_entity(object.obj.clone());
                object.is_placed = true;
            }
        }
    }
}

pub fn load_assets(mut commands: Commands, assets: Res<AssetServer>) {
    let ui_assets = Assets {
        font: assets.load("FiraCode-Regular.ttf"),
        icon: assets.load("Blueprint.png"),
    };

    commands.insert_resource(ui_assets);
}

pub fn spawn_blueprint(mut commands: EntityCommands, _assets: &Assets, pos: Vec2) {
    let group = EntityGroup {
        group: "todo!()".to_string(),
        color: client_profile::models::color::Color::Red,
        speed: 23.,
        directions: vec![Direction::Static],
        shape: Shape::Rectangle,
    };
    let location = Location {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let obj = Object::new(
        _assets,
        "Button 1".to_string(),
        "\nFirst button".to_string(),
        true,
        false,
        Vec2::new(pos.x, pos.y),
        Vec2::new(100., 50.),
        Entity { group, location },
    );
    obj.spawn(&mut commands);
}

pub fn spawn_box(
    mut commands: EntityCommands,
    _assets: &Assets,
    _windows: Res<Windows>,
    ass: Res<AssetServer>,
) {
    commands
        .insert(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        position: UiRect::new(
                            Val::Px(0.),
                            Val::Px(0.),
                            Val::Percent(80.),
                            Val::Px(0.),
                        ),
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Absolute,
                        size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                        ..default()
                    },
                    background_color: Color::rgb_u8(52, 73, 94).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                position: UiRect::new(
                                    Val::Percent(1.),
                                    Val::Px(0.),
                                    Val::Px(0.),
                                    Val::Px(0.),
                                ),
                                size: Size::new(Val::Percent(10.0), Val::Percent(80.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::rgba(0., 0., 0., 0.).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "+",
                                TextStyle {
                                    font: ass.load("FiraCode-Regular.ttf"),
                                    font_size: 250.0,
                                    color: Color::rgb(1.0, 1., 1.0),
                                },
                            ));
                        });
                });
        });
}
