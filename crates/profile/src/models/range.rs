use rand::distributions::Uniform;
use rand::prelude::Distribution;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Range {
    pub x: [f32; 2],
    pub y: [f32; 2],
    pub z: [f32; 2],
}

impl Range {
    pub fn uniform_x(&self) -> f32 {
        Self::uniform(self.x[0], self.x[1])
    }

    pub fn uniform_y(&self) -> f32 {
        Self::uniform(self.y[0], self.y[1])
    }
    pub fn uniform_z(&self) -> f32 {
        Self::uniform(self.z[0], self.z[1])
    }

    fn uniform(x_min: f32, x_max: f32) -> f32 {
        if x_min == x_max {
            return 0f32;
        }

        Uniform::from(x_min..x_max).sample(&mut rand::thread_rng())
    }
}
