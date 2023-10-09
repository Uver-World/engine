use std::env;

pub struct ApiSettings {
    pub hostname: String,
    pub port: u16,
    pub token: String,
}

impl ApiSettings {
    pub fn from_env() -> Self {
        let hostname: String = env::var_os("API_HOSTNAME")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let port = env::var_os("API_PORT")
            .unwrap()
            .to_str()
            .unwrap()
            .parse::<u16>()
            .unwrap();
        let token = env::var_os("API_TOKEN")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        Self {
            hostname,
            port,
            token,
        }
    }
}
