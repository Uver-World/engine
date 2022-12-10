use bevy::prelude::*;
use client_profile::models::direction::Direction;
use rand::distributions::Uniform;
use rand::prelude::Distribution;

use crate::assets::simulate_screen::retrieve_entities;
use crate::entities::ui_entity::UiEntity;
use crate::states::DisplayState;
use crate::ClientDisplay;

#[derive(Component)]
pub struct SimulateScreen;

impl Plugin for SimulateScreen {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(DisplayState::SimulateScreen).with_system(construct),
        )
        .add_system_set(SystemSet::on_exit(DisplayState::SimulateScreen).with_system(destroy))
        .add_system_set(
            SystemSet::on_update(DisplayState::SimulateScreen).with_system(update_status),
        )
        .add_system(keyboard_input);
    }
}

fn random_pos(ui_entity: &mut UiEntity) {
    let rand = Uniform::from(1..5).sample(&mut rand::thread_rng()); // TOP BOT, RIGHT, LEFT
    match rand {
        1 => {
            if ui_entity.y > -100.0 {
                ui_entity.y -= ui_entity.settings.group.speed;
            }
        }
        2 => {
            if ui_entity.y <= 100.0 {
                ui_entity.y += ui_entity.settings.group.speed;
            }
        }
        3 => {
            if ui_entity.x <= 300.0 {
                ui_entity.x += ui_entity.settings.group.speed;
            }
        }
        _ => {
            if ui_entity.x > 20.0 {
                ui_entity.x -= ui_entity.settings.group.speed;
            }
        }
    }
}

fn update_status(mut query: Query<(&mut Transform, &mut UiEntity)>) {
    for (mut style, mut ui_entity) in &mut query {
        match ui_entity.settings.group.direction.clone() {
            Direction::Random => {
                random_pos(&mut ui_entity);
            }
            Direction::Location(location) => {
                if ui_entity.x < location.x {
                    ui_entity.x += ui_entity.settings.group.speed;
                }
                if ui_entity.x > location.x {
                    ui_entity.x -= ui_entity.settings.group.speed;
                }
                if ui_entity.y < location.y {
                    ui_entity.y += ui_entity.settings.group.speed;
                }
                if ui_entity.y > location.y {
                    ui_entity.y -= ui_entity.settings.group.speed;
                }
            }
        }
        style.translation = Vec3::new(ui_entity.x, ui_entity.y, 1.);
    }
}

fn construct(mut commands: Commands, client: Res<ClientDisplay>) {
    let entities = retrieve_entities(client.profile.get_entities());
    let mut id = 0;

    for (entity, shape) in entities {
        commands
            .spawn(shape)
            .insert(UiEntity::from_entity(entity, id));
        id += 1;
    }
}

fn destroy(mut commands: Commands, query: Query<Entity, With<SimulateScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn keyboard_input(keys: Res<Input<KeyCode>>, mut app_state: ResMut<State<DisplayState>>) {
    if keys.just_pressed(KeyCode::B) {
        app_state.set(DisplayState::Blueprint).unwrap();
    }
}
