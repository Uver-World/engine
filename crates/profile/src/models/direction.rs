use serde::{Deserialize, Serialize};

use crate::models::location::Location;

#[derive(Serialize, Deserialize, Clone)]
pub enum Direction {
    Random,
    Location(Location),
    Static,
}
