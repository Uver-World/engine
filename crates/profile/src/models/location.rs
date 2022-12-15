use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct Location {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Location {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}
