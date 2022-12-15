use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use std::ops::RangeInclusive;

#[derive(Component)]
pub struct Camera3D {
    pub x: f32,
    pub y: f32,
    pub distance: f32,
    pub center: Vec3,
    pub rotate_sensitivity: f32,
    pub rotate_button: MouseButton,
    pub pan_sensitivity: f32,
    pub pan_button: MouseButton,
    pub pitch_range: RangeInclusive<f32>,
    pub zoom_sensitivity: f32,
    pub move_sensitivy: f32,
}

impl Default for Camera3D {
    fn default() -> Self {
        Camera3D {
            x: 0.0,
            y: std::f32::consts::FRAC_PI_2,
            distance: 5.0,
            center: Vec3::ZERO,
            rotate_sensitivity: 1.0,
            rotate_button: MouseButton::Left,
            pan_sensitivity: 1.0,
            pan_button: MouseButton::Right,
            pitch_range: 0.01..=3.13,
            zoom_sensitivity: 0.8,
            move_sensitivy: 300.,
        }
    }
}

pub struct Camera3DPlugin;
impl Camera3DPlugin {
    fn update_transform(
        mut query: Query<(&Camera3D, &mut Transform), (Changed<Camera3D>, With<Camera>)>,
    ) {
        for (camera, mut transform) in query.iter_mut() {
            let rot = Quat::from_axis_angle(Vec3::Y, camera.x)
                * Quat::from_axis_angle(-Vec3::X, camera.y);
            transform.translation = (rot * Vec3::Y) * camera.distance + camera.center;
            transform.look_at(camera.center, Vec3::Y);
        }
    }

    fn mouse_motion(
        time: Res<Time>,
        mut mouse_motion_events: EventReader<MouseMotion>,
        mouse_button_input: Res<Input<MouseButton>>,
        mut query: Query<(&mut Camera3D, &mut Transform, &mut Camera)>,
    ) {
        let mut delta = Vec2::ZERO;
        for event in mouse_motion_events.iter() {
            delta += event.delta;
        }
        for (mut camera, transform, _) in query.iter_mut() {
            if mouse_button_input.pressed(camera.rotate_button) {
                camera.x -= delta.x * camera.rotate_sensitivity * time.delta_seconds();
                camera.y -= delta.y * camera.rotate_sensitivity * time.delta_seconds();
                camera.y = camera
                    .y
                    .max(*camera.pitch_range.start())
                    .min(*camera.pitch_range.end());
            }

            if mouse_button_input.pressed(camera.pan_button) {
                let right_dir = transform.rotation * -Vec3::X;
                let up_dir = transform.rotation * Vec3::Y;
                let pan_vector = (delta.x * right_dir + delta.y * up_dir)
                    * camera.pan_sensitivity
                    * time.delta_seconds();
                camera.center += pan_vector;
            }
        }
    }

    fn key(
        time: Res<Time>,
        keys: Res<Input<KeyCode>>,
        mut query: Query<(&mut Camera3D, &mut Transform, &mut Camera)>,
    ) {
        for (mut camera, transform, _) in query.iter_mut() {
            if keys.pressed(KeyCode::Z) {
                let front_dir = transform.rotation * -Vec3::Z;
                let pan_vector = front_dir * camera.move_sensitivy * time.delta_seconds();
                camera.center += pan_vector;
            }
            if keys.pressed(KeyCode::S) {
                let front_dir = transform.rotation * Vec3::Z;
                let pan_vector = front_dir * camera.move_sensitivy * time.delta_seconds();
                camera.center += pan_vector;
            }
            if keys.pressed(KeyCode::Space) {
                let up_dir = transform.rotation * Vec3::Y;
                let pan_vector = up_dir * camera.move_sensitivy * time.delta_seconds();
                camera.center += pan_vector;
            }
            if keys.pressed(KeyCode::LShift) {
                let up_dir = transform.rotation * -Vec3::Y;
                let pan_vector = up_dir * camera.move_sensitivy * time.delta_seconds();
                camera.center += pan_vector;
            }
            if keys.pressed(KeyCode::Q) {
                let right_dir = transform.rotation * -Vec3::X;
                let pan_vector = right_dir * camera.move_sensitivy * time.delta_seconds();
                camera.center += pan_vector;
            }
            if keys.pressed(KeyCode::D) {
                let right_dir = transform.rotation * Vec3::X;
                let pan_vector = right_dir * camera.move_sensitivy * time.delta_seconds();
                camera.center += pan_vector;
            }
        }
    }

    fn zoom(
        mut mouse_wheel_events: EventReader<MouseWheel>,
        mut query: Query<&mut Camera3D, With<Camera>>,
    ) {
        let mut total = 0.0;
        for event in mouse_wheel_events.iter() {
            total += event.y
        }
        for mut camera in query.iter_mut() {
            camera.distance *= camera.zoom_sensitivity.powf(total);
        }
    }
}

impl Plugin for Camera3DPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::mouse_motion)
            .add_system(Self::key)
            .add_system(Self::zoom)
            .add_system(Self::update_transform);
    }
}
