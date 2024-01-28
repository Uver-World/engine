use serde::{Deserialize, Serialize};

use crate::models::Location;

use super::Range;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Direction {
    Random(Range),
    Location(Location),
    Static,
    Follow(Vec<String>),
    Escape(Vec<String>),
}
