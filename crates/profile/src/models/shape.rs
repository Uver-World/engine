use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Shape {
    Rectangle,
    Circle,
    Triangle,
    Ball,
}
