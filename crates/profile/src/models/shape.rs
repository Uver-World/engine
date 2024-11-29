use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum Shape {
    Rectangle,
    Circle,
    Triangle,
    Ball,
}

impl Shape {
    pub fn from_str(s: &str) -> Self {
        match s {
            "CIRCLE" => Self::Circle,
            "TRIANGLE" => Self::Triangle,
            "BALL" => Self::Ball,
            _ => Self::Rectangle,
        }
    }
}
