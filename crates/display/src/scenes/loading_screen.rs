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

fn update_status(query: Query<Entity, With<loading_screen::LoadingBar>>) {}

fn construct(mut commands: Commands, assets: Res<loading_screen::Assets>) {
    let mut node = commands.spawn();
    node.insert_bundle(NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::ColumnReverse,
            size: Size::new(Val::Percent(100.0), Val::Percent(90.0)),
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        color: Color::rgba(0., 0., 0., 0.).into(),
        ..default()
    });
    node.with_children(|parent| loading_screen::spawn_icon(parent.spawn(), &assets));
    node.with_children(|parent| loading_screen::spawn_loading_bar(parent.spawn(), &assets));
}

fn destroy(mut commands: Commands, query: Query<Entity, With<loading_screen::LoadingBar>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
