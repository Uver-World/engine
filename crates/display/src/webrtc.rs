use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use bevy::{prelude::*, render::view::screenshot::ScreenshotManager, window::PrimaryWindow};
use bevy_matchbox::{matchbox_socket::SingleChannel, MatchboxSocket};
use uverworld_packet::{
    packet::PacketType, set_simulation, set_tick_rate, update_entity, update_entity_group,
};

use crate::{
    api::Api,
    events::{
        set_simulation::{set_simulation_event, SetSimulation},
        set_tick_rate::{set_tick_rate_event, SetTickRateEvent},
        templates::{send_templates_event, GetTemplates},
        update_entity::{update_entity_event, UpdateEntityEvent},
        update_entity_group::{update_entity_group_event, UpdateEntityGroupEvent},
        ResetSimulation,
    },
};

#[derive(Resource, Clone)]
pub struct Images {
    image: Arc<Mutex<VecDeque<Image>>>,
}

impl Images {
    pub fn new() -> Self {
        let image = Arc::new(Mutex::new(VecDeque::new()));
        Self { image }
    }
}

pub struct WebRtc;

impl Plugin for WebRtc {
    fn build(&self, app: &mut App) {
        app.insert_resource(Images::new());
        app.add_systems(Startup, start_matchbox_socket)
            .add_systems(Update, (take_screenshot, check_peers, receive))
            .add_systems(Update, send_templates_event)
            .add_systems(Update, set_simulation_event)
            .add_systems(Update, update_entity_event)
            .add_systems(Update, update_entity_group_event)
            .add_systems(Update, set_tick_rate_event);
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
    let socket = MatchboxSocket::new_ggrs(room_url);
    commands.insert_resource(socket);
}

fn take_screenshot(
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    images: Res<Images>,
) {
    let image_manager = images.clone();
    let _ = screenshot_manager.take_screenshot(main_window.single(), move |image| {
        image_manager.image.lock().unwrap().push_back(image);
        println!("screenshot saved into images")
    });
}

fn check_peers(mut socket: ResMut<MatchboxSocket<SingleChannel>>, images: Res<Images>) {
    if socket.get_channel(0).is_err() {
        return; // we've already started
    }

    // Check for new connections
    socket.update_peers();
    eprintln!("peers connected = {}", socket.players().len());
    let mut images = images.image.lock().unwrap();
    while !images.is_empty() {
        let image = images.pop_front().unwrap();
        send_screenshot(image, &mut socket);
    }
}

fn send_screenshot(image: Image, socket: &mut MatchboxSocket<SingleChannel>) {
    let peers: Vec<_> = socket.connected_peers().collect();
    for peer in peers {
        socket.send(image.data.clone().into(), peer);
    }
    println!("screenshot sent to peers");
}

fn receive(
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut reset_simulation_event: EventWriter<ResetSimulation>,
    mut get_templates_event: EventWriter<GetTemplates>,
    mut set_simulation_event: EventWriter<SetSimulation>,
    mut update_entity_event: EventWriter<UpdateEntityEvent>,
    mut update_entity_group_event: EventWriter<UpdateEntityGroupEvent>,
    mut set_tick_rate_event: EventWriter<SetTickRateEvent>,
) {
    if socket.get_channel(0).is_err() {
        return; // we've already started
    }

    // Check for new connections
    socket.update_peers();
    let queue = socket.receive();

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
            PacketType::ResetSimulation => { reset_simulation_event.send(ResetSimulation); }
            PacketType::GetTemplates => { get_templates_event.send(GetTemplates); }
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
