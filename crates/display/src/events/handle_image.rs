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
    log::error,
};
use bevy_matchbox::{
    matchbox_socket::{PeerId, SingleChannel},
    MatchboxSocket,
};
use uverworld_packet::{packet::PacketType, Packet};
use image::imageops::FilterType;

#[derive(Event)]
pub struct HandleImage(pub uverworld_packet::image::Image);

pub fn handle_image(
    mut ev: EventReader<HandleImage>,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
) {
    let peers: Vec<_> = socket.connected_peers().collect();
    for image in ev.read() {
        let packet =
            uverworld_packet::create(PacketType::Image, uverworld_packet::image::encode(&image.0));
        send_screenshot(packet, peers.clone(), &mut socket);
    }
}

fn send_screenshot(packet: Packet, peers: Vec<PeerId>, socket: &mut MatchboxSocket<SingleChannel>) {
    let serialized: Box<[u8]> = uverworld_packet::serialize(&packet).into();
    for peer in peers {
        socket.send(serialized.clone(), peer);
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
            Ok(dyn_img) => {
                let dyn_img_resized = dyn_img.resize_exact(dyn_img.width() / 7, dyn_img.height() / 7, FilterType::Nearest);
                let image =
                    uverworld_packet::image::create_image(id, dyn_img_resized.width(), dyn_img_resized.height(), dyn_img_resized.as_bytes());
                let _ = sender
                    .lock()
                    .expect("Unable to acquire image_handler sender mutex lock")
                    .send(HandleImage(image))
                    .expect("Unable to send image_handler sender event");
            }
            Err(e) => error!("Cannot save screenshot, screen format cannot be understood: {e}"),
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
