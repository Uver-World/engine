use bevy::prelude::{SystemSet, App, Plugin, With, Entity, Query, Commands, Res, NodeBundle, default, Color, BuildChildren};
use bevy::ui::{Style, Display, FlexDirection, Val, Size, AlignItems, AlignContent};

use crate::states::DisplayState;
use crate::assets::blueprint;

pub struct Blueprint;

impl Plugin for Blueprint {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(DisplayState::Blueprint).with_system(construct))
            .add_system_set(SystemSet::on_exit(DisplayState::Blueprint).with_system(destroy))
            .add_system_set(SystemSet::on_update(DisplayState::Blueprint).with_system(update_status),);
    }
}

pub fn construct(mut commands: Commands, assets: Res<blueprint::Assets>) {
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
    node.with_children(|parent| blueprint::spawn_blueprint(parent.spawn_empty(), &assets));
    node.with_children(|parent| blueprint::spawn_box(parent.spawn_empty(), &assets));
}

pub fn destroy() {
    println!("Destroying blueprint scene");
}

pub fn update_status(_query: Query<Entity, With<blueprint::BlueprintBase>>) {}