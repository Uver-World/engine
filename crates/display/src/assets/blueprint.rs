use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

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
    return true;
    let rect: Rect = Rect {
        min: obj.pos,
        max: obj.pos + obj.size,
    };
    let rect2: Rect = Rect {
        min: obj.pos,
        max: obj.pos + obj.size,
    };
    println!("Rect: {:?} pos: {:?}", rect, pos);
    rect.contains(pos)
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
) {
    for (_entity, _, mut object, mut style, mut transform) in &mut query {
        if buttons.pressed(MouseButton::Left) {
            cursor_state.is_clicked = if cursor_state.is_clicked && !object.is_pressed {
                break;
            } else {
                true
            };
            if object.is_dragable {
                let wnd = windows.get_primary().unwrap();
                if let Some(screen_pos) = wnd.cursor_position() {
                    if !is_in_rect(object.clone(), screen_pos) {
                        println!(
                            "Not in range mouse: {:?} object: {:?}",
                            screen_pos, object.pos
                        );
                        continue;
                    }
                    cursor_state.is_dragging = true;
                    // screen_pos = get_world_pos(&windows, &q_camera, screen_pos);
                    object.pos = Vec2::new(screen_pos.x - object.size.x / 2., screen_pos.y);
                    println!("Drag to {:?}", object.pos);
                }
                style.position = object.get_rect();
                transform.translation = object.pos.extend(0.);
                object.is_pressed = true;
            }
        } else if buttons.just_released(MouseButton::Left) {
            if !cursor_state.is_dragging {
                cursor_state.is_clicked = false;
                continue;
            } else if !object.is_pressed {
                continue;
            } else if !cursor_state.is_clicked {
                (cursor_state.is_clicked, cursor_state.is_dragging) = (false, false);
                continue;
            }
            (
                cursor_state.is_clicked,
                cursor_state.is_dragging,
                object.is_pressed,
            ) = (false, false, false);
            let cpy = object.clone_at(object.init_pos);
            cpy.spawn(commands.spawn_empty());
            commands.entity(_entity).insert(cpy);
            client.profile.add_entity(object.obj.clone());
            object.is_placed = true;
        }
    }
}

/*                 Ex get entity                */
/*
        match client
            .profile
            .get_entity(|entity| entity.group.group == "Hello")
        {
            Some(entity) => {
                entity.group.group = "NoHello".to_string();
            }
            None => {}
        }
*/

pub fn load_assets(mut commands: Commands, assets: Res<AssetServer>) {
    let ui_assets = Assets {
        font: assets.load("FiraCode-Regular.ttf"),
        icon: assets.load("Blueprint.png"),
    };

    commands.insert_resource(ui_assets);
}

pub fn spawn_blueprint(
    mut commands: EntityCommands,
    _assets: &Assets,
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let group = EntityGroup {
        group: "todo!()".to_string(),
        color: client_profile::models::color::Color::Red,
        speed: 23.,
    };
    let location = Location { x: 0., y: 0. };
    let obj = Object::new(
        _assets,
        "Button 1".to_string(),
        "First button".to_string(),
        true,
        false,
        Vec2::new(732., 362.),
        Vec2::new(100., 50.),
        Entity { group, location },
        wnds,
        q_camera,
    );
    obj.spawn(commands);
}

pub fn spawn_box(
    mut commands: EntityCommands,
    assets: &Assets,
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
                            parent.spawn(ImageBundle {
                                image: ass.load("Plus.png").into(),
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                    ..Default::default()
                                },
                                ..Default::default()
                            });
                        });
                });
        })
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    position: UiRect::new(Val::Percent(85.), Val::Px(0.), Val::Px(0.), Val::Px(0.)),
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(15.0), Val::Percent(100.0)),
                    ..default()
                },
                background_color: Color::rgb_u8(52, 73, 94).into(),
                ..default()
            });
        });
}
