use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Location {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
