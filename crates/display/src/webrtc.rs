use std::sync::mpsc::channel;

use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use bevy_matchbox::{
    matchbox_socket::{ChannelConfig, MultipleChannels, WebRtcSocketBuilder},
    MatchboxSocket,
};
use uverworld_packet::{
    packet::PacketType, set_simulation, set_tick_rate, update_entity, update_entity_group,
};

use crate::{
    api::Api,
    events::{
        handle_image::{handle_image, take_screenshot, HandleImage, ImageHandler},
        set_simulation::{set_simulation_event, SetSimulation},
        set_tick_rate::{set_tick_rate_event, SetTickRateEvent},
        templates::{send_templates_event, GetTemplates},
        update_entity::{update_entity_event, UpdateEntityEvent},
        update_entity_group::{update_entity_group_event, UpdateEntityGroupEvent},
        ResetSimulation,
    },
    extensions::AppExtensions,
};

#[derive(ScheduleLabel, Clone, Debug, Eq, PartialEq, Hash)]
pub struct WebRtcSchedule;

pub struct WebRtc;

impl Plugin for WebRtc {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_matchbox_socket)
            .add_systems(WebRtcSchedule, receive)
            .add_systems(WebRtcSchedule, send_templates_event)
            .add_systems(WebRtcSchedule, set_simulation_event)
            .add_systems(WebRtcSchedule, update_entity_event)
            .add_systems(WebRtcSchedule, update_entity_group_event)
            .add_systems(WebRtcSchedule, set_tick_rate_event);

        let (handle_image_sender, handle_image_receiver) = channel();
        app.add_systems(WebRtcSchedule, (handle_image, take_screenshot));
        app.insert_resource(ImageHandler::new(handle_image_sender));
        app.add_event_channel::<HandleImage>(handle_image_receiver);
    }
}

fn start_matchbox_socket(mut commands: Commands, api: Res<Api>) {
    let peer = api.authenticate().unwrap();
    eprintln!("POST server_auth sent!");
    let room_url = format!(
        "ws://{hostname}:{port}/{id}",
        hostname = peer.signaling_hostname,
        port = peer.signaling_port,
        id = peer.room_id
    );
    eprintln!("connecting to matchbox server: {:?}", peer);
    let socket = WebRtcSocketBuilder::new(room_url)
        .add_channel(ChannelConfig::reliable())
        .add_reliable_channel()
        .add_unreliable_channel()
        .build();
    let matchbox_socket = MatchboxSocket::from(socket);
    commands.insert_resource(matchbox_socket);
}

fn receive(
    mut socket: ResMut<MatchboxSocket<MultipleChannels>>,
    mut reset_simulation_event: EventWriter<ResetSimulation>,
    mut get_templates_event: EventWriter<GetTemplates>,
    mut set_simulation_event: EventWriter<SetSimulation>,
    mut update_entity_event: EventWriter<UpdateEntityEvent>,
    mut update_entity_group_event: EventWriter<UpdateEntityGroupEvent>,
    mut set_tick_rate_event: EventWriter<SetTickRateEvent>,
) {
    // Check for new connections
    for (peer, state) in socket.update_peers() {
        info!("{peer}: {state:?}");
    }
    let reliable = socket.get_channel_mut(0).unwrap();
    let queue = reliable.receive();

    for (_, packet) in queue {
        let packet = uverworld_packet::deserialize(&packet);
        if packet.is_err() {
            eprintln!(
                "error happened whilst deserializing packet: {:?}",
                packet.unwrap_err()
            );
            continue;
        }
        let packet = packet.unwrap();

        match packet.packet_type() {
            PacketType::ResetSimulation => {
                reset_simulation_event.send(ResetSimulation);
            }
            PacketType::GetTemplates => {
                get_templates_event.send(GetTemplates);
            }
            PacketType::SetSimulation => {
                let template = set_simulation::deserialize(&packet.value)
                    .unwrap()
                    .template
                    .unwrap();
                set_simulation_event.send(SetSimulation(template));
            }
            PacketType::UpdateEntity => {
                let update_entity = update_entity::deserialize(&packet.value).unwrap();
                update_entity_event.send(UpdateEntityEvent(update_entity));
            }
            PacketType::UpdateEntityGroup => {
                let update_entity = update_entity_group::deserialize(&packet.value).unwrap();
                update_entity_group_event.send(UpdateEntityGroupEvent(update_entity));
            }
            PacketType::SetTickRate => {
                let set_tick_rate = set_tick_rate::deserialize(&packet.value).unwrap();
                set_tick_rate_event.send(SetTickRateEvent(set_tick_rate));
            }
            _ => eprintln!("packet not supported"),
        };
    }
}

pub fn close_matchbox_socket(api: &Api) {
    let _ = api.server_disconnect();
    eprintln!("POST server_disconnect sent!");
}
