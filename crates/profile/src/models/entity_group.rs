use serde::{Deserialize, Serialize};

use crate::models::{Color, Direction, Shape};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EntityGroup {
    pub name: String,
    pub color: Color,
    pub speed: f32,
    pub directions: Vec<Direction>,
    pub shape: Shape,
}
