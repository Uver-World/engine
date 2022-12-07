use bevy::prelude::*;

use crate::assets::simulate_screen::spawn_entities;
use crate::entities::entity;
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

fn update_status(mut query: Query<(&mut Style, &mut entity::Entity)>) {
    for (mut style, mut entity) in &mut query {
        entity.left += 10.0;
        style.position = entity.get_rect();
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
