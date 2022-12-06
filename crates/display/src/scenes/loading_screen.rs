use bevy::prelude::*;

use crate::assets::loading_screen;
use crate::states::DisplayState;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(DisplayState::LoadingScreen).with_system(construct))
            .add_system_set(SystemSet::on_exit(DisplayState::LoadingScreen).with_system(destroy))
            .add_system_set(
                SystemSet::on_update(DisplayState::LoadingScreen).with_system(update_status),
            );
    }
}

fn update_status(_query: Query<Entity, With<loading_screen::LoadingBar>>) {}

fn construct(mut commands: Commands, assets: Res<loading_screen::Assets>) {
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
    node.with_children(|parent| loading_screen::spawn_icon(parent.spawn_empty(), &assets));
    node.with_children(|parent| loading_screen::spawn_loading_bar(parent.spawn_empty(), &assets));
}

fn destroy(mut commands: Commands, query: Query<Entity, With<loading_screen::LoadingBar>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
