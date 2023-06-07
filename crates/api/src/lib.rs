use models::{Peer, UserId};

pub mod models;

pub fn server_auth(hostname: &str, port: u16, server_token: &str) -> Result<Peer, String> {
    let path = format!("http://{hostname}:{port}/user/server_authenticate");
    let client = reqwest::blocking::Client::new();

    client
        .post(path)
        .header("X-User-Token", server_token)
        .send()
        .unwrap()
        .json::<Peer>()
        .map_err(|err| err.to_string())
}

pub fn server_disconnect(hostname: &str, port: u16, server_token: &str) -> Result<bool, String> {
    let path = format!("http://{hostname}:{port}/user/server_disconnect");
    let client = reqwest::blocking::Client::new();

    client
        .post(path)
        .header("X-User-Token", server_token)
        .send()
        .unwrap()
        .json::<bool>()
        .map_err(|err| err.to_string())
}

pub fn has_access(
    hostname: &str,
    port: u16,
    server_token: &str,
    user_id: UserId,
) -> Result<bool, String> {
    let path = format!("http://{hostname}:{port}/user/has_access");
    let client = reqwest::blocking::Client::new();

    client
        .post(path)
        .body(serde_json::to_string(&user_id).unwrap())
        .header("X-User-Token", server_token)
        .send()
        .unwrap()
        .json::<bool>()
        .map_err(|err| err.to_string())
}
