use serde::{Deserialize, Serialize};

use crate::models::Location;

use super::{Range, SightRadius};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Direction {
    Random(Range),
    Location(Location),
    Static,
    Follow(SightRadius, Vec<String>),
    Escape(SightRadius, Vec<String>),
}
