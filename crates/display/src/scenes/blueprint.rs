use bevy::prelude::{SystemSet, App, Plugin, With, Entity, Query, Commands, Res, NodeBundle, default, Color, BuildChildren, KeyCode, Input, ResMut, State, Component, DespawnRecursiveExt, Camera, GlobalTransform};
use bevy::ui::{Style, Display, FlexDirection, Val, Size, AlignItems, AlignContent};
use bevy::window::Windows;

use crate::ClientDisplay;
use crate::assets::blueprint::drag;
use crate::assets::blueprint_structure::{BlueprintBase, Object};
use crate::states::DisplayState;
use crate::assets::blueprint;

#[derive(Component)]
pub struct Blueprint;

impl Plugin for Blueprint {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(DisplayState::Blueprint).with_system(construct))
            .add_system_set(SystemSet::on_exit(DisplayState::Blueprint).with_system(destroy))
            // .add_system_set(SystemSet::on_update(DisplayState::Blueprint).with_system(update_status),)
            .add_system_set(SystemSet::on_update(DisplayState::Blueprint).with_system(drag),)
            .add_system(keyboard_input);
    }
}

pub fn construct(mut commands: Commands, assets: Res<blueprint::Assets>, windows: Res<bevy::window::Windows>, wnds: Res<Windows>, q_camera: Query<(&Camera, &GlobalTransform)>) {
    let mut node = commands.spawn(Blueprint);
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
    node.with_children(|parent| blueprint::spawn_blueprint(parent.spawn_empty(), &assets, wnds, q_camera));
    node.with_children(|parent| blueprint::spawn_box(parent.spawn_empty(), &assets, windows));
}

pub fn destroy(mut commands: Commands, query: Query<Entity, With<Blueprint>>, query2: Query<Entity, With<Object>>, client: ResMut<ClientDisplay>,) {
    client.profile.save();
    println!("destroying blueprint");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in query2.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_status(_query: Query<Entity, With<BlueprintBase>>) {}

pub fn keyboard_input(keys: Res<Input<KeyCode>>, mut app_state: ResMut<State<DisplayState>>) {
    if keys.just_pressed(KeyCode::S) {
        match app_state.current() {
            DisplayState::Blueprint => {
                app_state.set(DisplayState::SimulateScreen).unwrap();
            }
            DisplayState::SimulateScreen => {},
            DisplayState::LoadingScreen => {},
            DisplayState::Menu => {},
        }
    }
}
