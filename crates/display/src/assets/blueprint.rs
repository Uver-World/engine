use bevy::render::camera::RenderTarget;
use bevy::{ecs::system::EntityCommands};
use bevy::prelude::*;

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
    pub bund: ButtonBundle
}

impl Object {
    pub fn new(asset: &Assets, name: String, description: String, is_dragable: bool, is_pressed: bool, pos: Vec2) -> Self {
        Self {
            asset: asset.clone().into(),
            name,
            description,
            is_dragable,
            is_pressed,
            pos,
            bund: ButtonBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.)),
                image: asset.icon.clone().into(),
                ..default()
            }
        }
    }

    pub fn spawn(&self, mut commands: EntityCommands) {
        commands.insert(
            self.bund.clone()
        ).with_children(|parent| {
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
                ])
            );
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
            bund: ButtonBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(pos.x),
                        bottom: Val::Px(pos.y),
                        ..Default::default()
                    },
                    ..default()
                },
                transform: Transform::from_scale(Vec3::new(1., 1., 1.)),
                // transform: Transform::from_translation(Vec3::new(pos.x,pos.y,0.)),
                image: self.asset.icon.clone().into(),
                ..default()
            }
        }
    }
}

pub fn drag(
    mut _commands: Commands,
    q_camera: Query<(&Camera, &GlobalTransform),With<Camera2d>>,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut query: Query<(Entity, With<Object>, &mut Object)>,
    mut query2: Query<&mut Transform, With<Object>>
) {
    for (mut _entity, _, mut _object) in &mut query {
        if buttons.pressed(MouseButton::Left) {
            if _object.is_dragable {
                _object.is_pressed = true;
                let mut transform = query2.single_mut();
                let (camera, camera_transform) = q_camera.single();
                let wnd = if let RenderTarget::Window(id) = camera.target {
                    windows.get(id).unwrap()
                } else {
                    windows.get_primary().unwrap()
                };
                if let Some(screen_pos) = wnd.cursor_position() {
                    let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
                    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
                    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
                    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
                    let world_pos: Vec2 = world_pos.truncate();
                    _object.pos = Vec2::new(world_pos.x, world_pos.y);
                    // println!("transform base = {:?}", transform.translation);
                    // _object.bund.style.position.left = Val::Px(_object.pos.x);
                    // _object.bund.style.position.bottom = Val::Px(_object.pos.y);
                    // _object.bund.transform = Transform::from_translation(Vec3::new(_object.pos.x, _object.pos.y, 0.));
                    // transform = Transform::from_translation(Vec3::new(screen_pos.x, screen_pos.y, 0.));
                    let cpy = _object.clone_at(world_pos);
                    // _commands.entity(_entity).despawn_recursive();
                    _commands.entity(_entity).with_children(|parent| cpy.spawn(parent.spawn_empty()));
                    _commands.entity(_entity).insert(cpy);
                    println!("Drag to {:?}", _object.pos);
                }
            }
        } else if buttons.just_released(MouseButton::Left) {
            _object.is_pressed = false;
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

pub fn spawn_blueprint(mut commands: EntityCommands, _assets: &Assets) {
    let obj = Object::new(_assets, "Button 1".to_string(), "First button".to_string(), true, false, Vec2::new(0., 0.));
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
