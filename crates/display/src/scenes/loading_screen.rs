use bevy::prelude::*;
use bevy_rapier3d::plugin::{RapierConfiguration, TimestepMode};

use crate::assets::loading_screen;
use crate::states::DisplayState;
use crate::ClientDisplay;

#[derive(Component)]
pub struct LoadingScreen;

impl Plugin for LoadingScreen {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(DisplayState::LoadingScreen),
            (construct, update_rapier),
        )
        .add_systems(OnExit(DisplayState::LoadingScreen), destroy)
        .add_systems(
            Update,
            update_status.run_if(in_state(DisplayState::LoadingScreen)),
        );
    }
}

fn update_rapier(
    client_display: Res<ClientDisplay>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.timestep_mode = TimestepMode::Fixed {
        dt: 1.0 / client_display.tick_rate,
        substeps: 1,
    };
}

fn update_status(
    mut query: Query<(&mut Style, &mut loading_screen::LoadingBar)>,
    mut app_state: ResMut<NextState<DisplayState>>,
) {
    for (mut style, mut loading_bar) in query.iter_mut() {
        if loading_bar.val < 100.0 {
            let r = 1.0;
            if loading_bar.val + r > 100.0 {
                loading_bar.val = 100.0;
            } else {
                loading_bar.val += r;
            }
            style.width = Val::Percent(loading_bar.val);
        } else {
            app_state.set(DisplayState::SimulateScreen);
        }
    }
}

fn construct(mut commands: Commands, assets: Res<loading_screen::Assets>) {
    let mut node = commands.spawn(LoadingScreen);
    node.insert(Camera2dBundle::default());
    node.insert(NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            ..default()
        },
        background_color: Color::srgba(0., 0., 0., 0.).into(),
        ..default()
    });
    node.with_children(|parent| loading_screen::spawn_icon(parent.spawn_empty(), &assets));
    node.with_children(|parent| loading_screen::spawn_loading_bar(parent.spawn_empty(), &assets));
}

fn destroy(mut commands: Commands, query: Query<Entity, With<LoadingScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
