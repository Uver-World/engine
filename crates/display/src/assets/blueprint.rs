use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

use crate::ClientDisplay;
use client_profile::models::entity::{Entity, EntityGroup};
use client_profile::models::location::Location;

#[derive(Component)]
pub struct BlueprintBase;

#[derive(Resource, Clone)]
pub struct Assets {
    pub font: Handle<Font>,
    pub icon: Handle<Image>,
}

#[derive(Component)]
pub struct Object {
    pub asset: Assets,
    pub name: String,
    pub description: String,
    pub is_dragable: bool,
    pub is_pressed: bool,
    pub pos: Vec2,
    pub init_pos: Vec2,
    pub bund: ImageBundle,
    pub size: Vec2,
    pub is_placed: bool,
    pub obj: Entity,
}

impl Object {
    pub fn new(
        asset: &Assets,
        name: String,
        description: String,
        is_dragable: bool,
        is_pressed: bool,
        pos: Vec2,
        size: Vec2,
        obj: Entity,
        wnds: Res<Windows>,
        q_camera: Query<(&Camera, &GlobalTransform)>
    ) -> Self {
        let world_pos = get_world_pos(wnds, q_camera, pos);
        Self {
            asset: asset.clone().into(),
            name,
            description,
            is_dragable,
            is_pressed,
            pos: pos,
            init_pos: world_pos,
            bund: ImageBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(world_pos.x),
                        bottom: Val::Px(world_pos.y),
                        ..default()
                    },
                    size: Size::new(Val::Px(size.x), Val::Px(size.y)),
                    ..default()
                },
                // transform: Transform::default(),
                transform: Transform::from_translation(Vec3::new(1., 1., 0.)),
                image: asset.icon.clone().into(),
                ..default()
            },
            size,
            is_placed: false,
            obj,
        }
    }

    pub fn spawn(&self, mut commands: EntityCommands) {
        commands.insert(self.bund.clone()).with_children(|parent| {
            parent.spawn(TextBundle::from_sections([
                TextSection::new(
                    self.name.clone(),
                    TextStyle {
                        font: self.asset.font.clone(),
                        font_size: 20.0,
                        color: Color::RED,
                    },
                ),
                TextSection::new(
                    self.description.clone(),
                    TextStyle {
                        font: self.asset.font.clone(),
                        font_size: 15.0,
                        color: Color::BLUE,
                    },
                ),
            ]));
        });
    }

    pub fn clone_at(&self, pos: Vec2) -> Self {
        Self {
            asset: self.asset.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            is_dragable: self.is_dragable,
            is_pressed: self.is_pressed,
            pos,
            init_pos: pos,
            bund: ImageBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(pos.x),
                        bottom: Val::Px(pos.y),
                        ..default()
                    },
                    size: Size::new(Val::Px(self.size.x), Val::Px(self.size.y)),
                    ..default()
                },
                // transform: Transform::from_scale(Vec3::new(1., 1., 1.)),
                transform: Transform::from_translation(Vec3::new(pos.x,pos.y,0.)),
                image: self.asset.icon.clone().into(),
                ..default()
            },
            size: self.size,
            is_placed: false,
            obj: self.obj.clone(),
        }
    }

    pub fn get_rect(&self) -> UiRect {
        UiRect {
            left: Val::Px(self.pos.x),
            bottom: Val::Px(self.pos.y),
            ..default()
        }
    }
}

pub fn get_world_pos(
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
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

/*
use bevy::prelude::*;

fn my_system(entities: &[Entity],
             mut mouse_moved_events: ResMut<Events<MouseMoved>>) {
    for event in mouse_moved_events.iter() {
        let element = event.element;
        // element est l'entité sur laquelle le curseur se déplace
    }
}
*/

pub fn drag(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut client: ResMut<ClientDisplay>,
    mut query: Query<(bevy::prelude::Entity, With<Object>, &mut Object, &mut Style)>,
) {
    for (_entity, _, mut object, mut style) in &mut query {
        if buttons.pressed(MouseButton::Left) {
            println!("left pressed");
            if object.is_dragable {
                let wnd = windows.get_primary().unwrap();
                if let Some(screen_pos) = wnd.cursor_position() {
                    println!("move");
                    if !(screen_pos.x >= object.pos.x && screen_pos.x <= object.pos.x + object.size.x) && (screen_pos.y >= object.pos.y && screen_pos.y <= object.pos.y + object.size.y) {
                        println!("Not in range mouse: {:?} object: {:?}", screen_pos, object.pos);
                        continue;
                    }
                    println!("in range mouse: {:?} object: {:?}", screen_pos, object.pos);
                    object.pos = Vec2::new(screen_pos.x - object.size.x, screen_pos.y);
                }
                style.position = object.get_rect();
                object.is_pressed = true;
            }
        } else if buttons.just_released(MouseButton::Left) {
            println!("Clone");
            let cpy = object.clone_at(object.init_pos);
            commands
                .entity(_entity)
                .with_children(|parent| cpy.spawn(parent.spawn_empty()));
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

pub fn spawn_blueprint(mut commands: EntityCommands, _assets: &Assets, wnds: Res<Windows>, q_camera: Query<(&Camera, &GlobalTransform)>) {
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
        q_camera
    );
    // let obj2 = Object::new(_assets, "Button 2".to_string(), "Second button".to_string(), true, false, Vec2::new(500., 100.));
    commands.with_children(|parent| obj.spawn(parent.spawn_empty()));
    // commands.with_children(|parent| obj2.spawn(parent.spawn_empty()));
    commands.insert(obj);
    // commands.insert(obj2);
    commands.insert(ImageBundle {
        style: Style {
            align_self: AlignSelf::Center,
            position_type: PositionType::Absolute,
            ..default()
        },
        transform: Transform::from_scale(Vec3::new(2.5, 2.5, 2.5)),
        image: _assets.icon.clone().into(),
        ..default()
    });
}

pub fn spawn_box(mut commands: EntityCommands, _assets: &Assets, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    commands.insert(NodeBundle {
        style: Style {
            position: UiRect::new(Val::Px(0.), Val::Px(0.), Val::Px(0.), Val::Px(0.)),
            position_type: PositionType::Absolute,
            size: Size::new(Val::Px(window.width() * 0.15), Val::Px(window.height())),
            ..default()
        },
        background_color: Color::rgba(1., 1., 1., 0.6).into(),
        ..default()
    });
}
