use serde::{Deserialize, Serialize};
use serde_json::Result;

use crate::models::{Color, Direction, Shape};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EntityGroup {
    pub name: String,
    pub color: Color,
    pub speed: f32,
    pub directions: Vec<Direction>,
    pub shape: Shape,
    pub gravity: f32,
    pub texture_id: String,
}

impl EntityGroup {
    pub fn from_str(value: &str) -> Result<Self> {
        serde_json::from_str(value)
    }

    pub fn to_str(&self) -> Result<String> {
        serde_json::to_string(self)
    }
}
