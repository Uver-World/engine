use std::fs;

use bevy::ecs::{
    event::{Event, EventReader, EventWriter},
    system::ResMut,
};
use bevy_matchbox::{matchbox_socket::MultipleChannels, MatchboxSocket};
use uverworld_packet::templates::{Template, Templates};

#[derive(Event)]
pub struct SendTemplates(Templates);

#[derive(Event)]
pub struct GetTemplates;

pub fn get_templates_event(
    ev: EventReader<GetTemplates>,
    mut send_templates_event: EventWriter<SendTemplates>,
) {
    if ev.is_empty() {
        return;
    }

    let templates = retrieve_templates();
    println!("templates = {:?}", templates);
    send_templates_event.send(SendTemplates(templates));
}

pub fn send_templates_event(
    mut ev: EventReader<SendTemplates>,
    mut socket: ResMut<MatchboxSocket<MultipleChannels>>,
) {
    let peers: Vec<_> = socket.connected_peers().collect();
    let reliable = socket.get_channel_mut(0).unwrap();
    for events in ev.read() {
        let serialized_templates = uverworld_packet::templates::encode(&events.0);

        for peer in peers.clone() {
            reliable.send(serialized_templates.clone().into(), peer);
        }
    }
}

fn retrieve_templates() -> Templates {
    let template_folder = fs::read_dir("templates/").unwrap();
    let mut templates = Vec::new();

    for template in template_folder {
        let template = template.unwrap();
        templates.push(Template {
            file_name: template.file_name().to_str().unwrap().into(),
            file_content: fs::read_to_string(template.path()).unwrap(),
        });
    }

    Templates { templates }
}
