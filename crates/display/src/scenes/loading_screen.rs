use bevy::prelude::*;

use crate::assets::loading_screen;
use crate::states::DisplayState;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(DisplayState::LoadingScreen).with_system(construct))
            .add_system_set(SystemSet::on_exit(DisplayState::LoadingScreen).with_system(destroy));
    }
}

fn construct(mut commands: Commands, assets: Res<loading_screen::Assets>) {
    loading_screen::spawn_icon(commands.spawn(), &assets);
}

fn destroy(mut commands: Commands, query: Query<Entity, With<loading_screen::LoadingBar>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
