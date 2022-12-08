use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    pub fn get_value(&self) -> [f32; 3] {
        match self {
            Self::Red => [255.0, 0.0, 0.0],
            Self::Green => [0.0, 255.0, 0.0],
            Self::Blue => [0.0, 0.0, 255.0],
        }
    }

    pub fn red(&self) -> f32 {
        self.get_value()[0]
    }

    pub fn green(&self) -> f32 {
        self.get_value()[1]
    }

    pub fn blue(&self) -> f32 {
        self.get_value()[2]
    }
}
