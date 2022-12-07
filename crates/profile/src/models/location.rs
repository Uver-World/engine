use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub x: f32,
    pub y: f32,
}
