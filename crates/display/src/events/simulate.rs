use bevy::ecs::{
    event::{Event, EventReader},
    system::ResMut,
};
use bevy::state::state::NextState;

use crate::states::DisplayState;

#[derive(Event)]
pub struct ResetSimulation;

pub fn reset_simulation_event(
    mut app_state: ResMut<NextState<DisplayState>>,
    ev: EventReader<ResetSimulation>,
) {
    if ev.is_empty() {
        return;
    }
    app_state.set(DisplayState::LoadingScreen);
}
