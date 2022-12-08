use bevy::prelude::*;
use client_profile::models::direction::Direction;
use rand::distributions::Uniform;
use rand::prelude::Distribution;

use crate::assets::simulate_screen::spawn_entities;
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
        );
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

fn update_status(mut query: Query<(&mut Style, &mut UiEntity)>) {
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

        style.position = ui_entity.get_rect();
    }
}

fn construct(mut commands: Commands, client: Res<ClientDisplay>) {
    let mut node = commands.spawn(SimulateScreen);
    node.insert(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
            ..default()
        },
        background_color: Color::rgb(255., 255., 255.).into(),
        ..default()
    })
    .with_children(|parent| {
        spawn_entities(
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(99.0), Val::Percent(99.0)),
                    ..default()
                },
                background_color: Color::rgb(0., 0., 0.).into(),
                ..default()
            }),
            client.profile.get_entities(),
        );
    });
}

fn destroy(mut commands: Commands, query: Query<Entity, With<SimulateScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
