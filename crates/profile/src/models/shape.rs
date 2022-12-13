use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum Shape {
    Rectangle,
    Circle,
    Triangle,
    Ball,
}
