use bevy::ecs::{
    event::{Event, EventReader},
    system::ResMut,
};
use bevy_rapier3d::plugin::{RapierConfiguration, TimestepMode};
use uverworld_packet::set_tick_rate::SetTickRate;

use crate::ClientDisplay;

fn update_rapier(client_display: &mut ClientDisplay, rapier_config: &mut RapierConfiguration) {
    rapier_config.timestep_mode = TimestepMode::Fixed {
        dt: 1.0 / client_display.tick_rate,
        substeps: 1,
    };
}

#[derive(Event)]
pub struct SetTickRateEvent(pub SetTickRate);

pub fn set_tick_rate_event(
    mut ev: EventReader<SetTickRateEvent>,
    mut client: ResMut<ClientDisplay>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    for event in ev.iter() {
        let new_tick_rate = event.0.tick_rate;

        client.tick_rate = new_tick_rate;
        update_rapier(&mut client, &mut rapier_config);
    }
}
