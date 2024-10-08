use std::sync::{mpsc::Sender, Arc, Mutex};

use bevy::{
    ecs::{
        entity::Entity,
        event::{Event, EventReader},
        query::With,
        system::{Query, ResMut, Resource},
    },
    render::view::screenshot::ScreenshotManager,
    window::PrimaryWindow,
};
use bevy_matchbox::{
    matchbox_socket::{MultipleChannels, PeerId, WebRtcChannel},
    MatchboxSocket,
};
use image::imageops::FilterType;
use uverworld_packet::{packet::PacketType, Packet};

#[derive(Event)]
pub struct HandleImage(pub uverworld_packet::image::Image);

pub fn handle_image(
    mut ev: EventReader<HandleImage>,
    mut socket: ResMut<MatchboxSocket<MultipleChannels>>,
) {
    let peers: Vec<_> = socket.connected_peers().collect();
    let mut unreliable = socket.get_channel_mut(1).unwrap();
    for image in ev.read() {
        let packet =
            uverworld_packet::create(PacketType::Image, uverworld_packet::image::encode(&image.0));
        send_screenshot(packet, peers.clone(), &mut unreliable);
        println!("screenshot sent!")
    }
}

fn send_screenshot(packet: Packet, peers: Vec<PeerId>, unreliable: &mut WebRtcChannel) {
    let serialized: Box<[u8]> = uverworld_packet::serialize(&packet).into();
    for peer in peers {
        unreliable.send(serialized.clone(), peer);
        println!("screenshot sent to peer");
    }
}

pub fn take_screenshot(
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    mut image_handler: ResMut<ImageHandler>,
) {
    let sender = image_handler.sender.clone();
    let id = image_handler.id;
    image_handler.id += 1;
    let _ = screenshot_manager.take_screenshot(main_window.single(), move |image| {
        match image.try_into_dynamic() {
            Ok(dyn_image) => {
                let dyn_image = dyn_image.resize(100, 100, FilterType::Nearest);

                let image = uverworld_packet::image::create_image(
                    id,
                    dyn_image.width(),
                    dyn_image.height(),
                    &dyn_image.as_bytes(),
                );
                let _ = sender
                    .lock()
                    .expect("Unable to acquire image_handler sender mutex lock")
                    .send(HandleImage(image))
                    .expect("Unable to send image_handler sender event");
                println!("sending image");
            }
            Err(e) => {
                eprintln!("cannot convert image to dyamic image: {e:?}")
            }
        }
    });
}

#[derive(Resource, Clone)]
pub struct ImageHandler {
    sender: Arc<Mutex<Sender<HandleImage>>>,
    id: u64,
}

impl ImageHandler {
    pub fn new(sender: Sender<HandleImage>) -> Self {
        Self {
            sender: Arc::new(Mutex::new(sender)),
            id: 0,
        }
    }
}
