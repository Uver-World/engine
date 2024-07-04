use std::fs;

use bevy::ecs::{
    event::{Event, EventReader, EventWriter},
    system::ResMut,
};
use bevy_matchbox::{matchbox_socket::MultipleChannels, MatchboxSocket};
use uverworld_packet::textures::{Texture, Textures};

#[derive(Event)]
pub struct SendTextures(Textures);

#[derive(Event)]
pub struct GetTextureEvent;

pub fn get_textures_event(
    ev: EventReader<GetTextureEvent>,
    mut send_textures_event: EventWriter<SendTextures>,
) {
    if ev.is_empty() {
        return;
    }

    let textures = retrieve_textures();
    println!("textures = {:?}", textures);
    send_textures_event.send(SendTextures(textures));
}

pub fn send_textures_event(
    mut ev: EventReader<SendTextures>,
    mut socket: ResMut<MatchboxSocket<MultipleChannels>>,
) {
    let peers: Vec<_> = socket.connected_peers().collect();
    let reliable = socket.get_channel_mut(0).unwrap();
    for events in ev.read() {
        let serialized_textures = uverworld_packet::textures::encode(&events.0);

        for peer in peers.clone() {
            reliable.send(serialized_textures.clone().into(), peer);
        }
    }
}

fn retrieve_textures() -> Textures {
    let texture_folder = fs::read_dir("textures/").unwrap();
    let mut textures = Vec::new();

    for texture in texture_folder {
        let texture = texture.unwrap();
        textures.push(Texture {
            id: texture.file_name().to_str().unwrap().into(),
            description: String::new(),
        });
    }

    Textures { textures }
}
