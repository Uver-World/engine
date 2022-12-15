use serde::{Deserialize, Serialize};

use crate::models::location::Location;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Direction {
    Random,
    Location(Location),
    Static,
    Follow(Vec<String>),
    Escape(Vec<String>),
}
