use bevy::ecs::{
    event::{Event, EventReader},
    schedule::NextState,
    system::ResMut,
};
use client_profile::Profile;
use uverworld_packet::templates::Template;

use crate::{states::DisplayState, ClientDisplay};

#[derive(Event)]
pub struct SetSimulation(pub Template);

pub fn set_simulation_event(
    mut app_state: ResMut<NextState<DisplayState>>,
    mut ev: EventReader<SetSimulation>,
    mut client: ResMut<ClientDisplay>,
) {
    for events in ev.read() {
        let template = &events.0;
        let new_profile = Profile::custom_load(&template.file_content);
        match new_profile {
            Ok(new_profile) => {
                client.settings.profile = new_profile;
                app_state.set(DisplayState::LoadingScreen);
            }
            Err(err) => eprintln!("An error occured whilst SetSimulation event handling: {err:#?}"),
        }
    }
}
