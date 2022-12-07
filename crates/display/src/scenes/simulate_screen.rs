use bevy::prelude::*;

use crate::assets::simulate_screen;
use crate::entities::entity;
use crate::states::DisplayState;

pub struct SimulateScreenPlugin;

impl Plugin for SimulateScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(DisplayState::SimulateScreen).with_system(construct))
            .add_system_set(SystemSet::on_exit(DisplayState::SimulateScreen).with_system(destroy))
            .add_system_set(
                SystemSet::on_update(DisplayState::SimulateScreen).with_system(update_status),
            );
    }
}

fn update_status(_query: Query<Entity, With<entity::Entity>>) {
    
}

fn construct(mut commands: Commands) {
    let mut node = commands.spawn_empty();
    node.insert(NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            ..default()
        },
        background_color: Color::rgba(0., 0., 0., 0.).into(),
        ..default()
    });

    for id in 0..9 {
        node.with_children(|parent| simulate_screen::spawn_entity(parent.spawn_empty(), id));
    }
}

fn destroy(mut commands: Commands, query: Query<Entity, With<entity::Entity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
