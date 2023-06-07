use bevy::ecs::system::Commands;
use bevy::prelude::{Res, ResMut};
use bevy_matchbox::prelude::SingleChannel;
use bevy_matchbox::MatchboxSocket;

use crate::api::Api;

pub fn start_matchbox_socket(mut commands: Commands, api: Res<Api>) {
    let peer = api.authenticate().unwrap();
    eprintln!("POST server_auth sent!");
    let room_url = format!(
        "ws://{hostname}:{port}/{id}",
        hostname = peer.signaling_hostname,
        port = peer.signaling_port,
        id = peer.room_id
    );
    eprintln!("connecting to matchbox server: {:?}", peer);
    commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
}

pub fn close_matchbox_socket(api: &Api) {
    let _ = api.server_disconnect();
    eprintln!("POST server_disconnect sent!");
}

pub fn check_peers(mut socket: ResMut<MatchboxSocket<SingleChannel>>) {
    if socket.get_channel(0).is_err() {
        return; // we've already started
    }

    // Check for new connections
    socket.update_peers();
    eprintln!("peers connected = {}", socket.players().len());
}
