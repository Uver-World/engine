use serde::{Deserialize, Serialize};

use super::Location;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SightRadius(f32);

impl SightRadius {
    pub fn is_in_sight(&self, entity_location: Location, target_location: Location) -> bool {
        let distance = Self::compute_distance(entity_location, target_location);

        distance < self.0
    }

    fn compute_distance(entity_location: Location, target_location: Location) -> f32 {
        let dx = entity_location.x - target_location.x;
        let dy = entity_location.y - target_location.y;
        let dz = entity_location.z - target_location.z;

        (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
    }
}
