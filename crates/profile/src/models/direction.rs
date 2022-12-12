use serde::{Deserialize, Serialize};

use crate::models::location::Location;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum Direction {
    Random,
    Location(Location),
    Follow(Vec<String>),
    Escape(Vec<String>),
}
