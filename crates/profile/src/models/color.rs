use hex_color::HexColor;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum Color {
    Red,
    Green,
    Blue,
    Cyan,
    Pink,
    Purple,
    Orange,
    Magenta,
    Brown,
    Gray,
    Lime,
    Custom(String),
}

impl Color {
    pub fn get_value(&self) -> [u8; 3] {
        match self {
            Self::Red => [255, 0, 0],
            Self::Green => [0, 128, 0],
            Self::Blue => [0, 0, 255],
            Self::Cyan => [0, 255, 255],
            Self::Pink => [255, 192, 203],
            Self::Purple => [255, 20, 147],
            Self::Orange => [255, 165, 0],
            Self::Magenta => [255, 0, 255],
            Self::Brown => [165, 42, 42],
            Self::Gray => [128, 128, 128],
            Self::Lime => [0, 255, 0],
            Self::Custom(value) => {
                let color = HexColor::parse_rgb(value).unwrap_or(HexColor::GRAY);
                [color.r, color.g, color.b]
            }
        }
    }

    pub fn red(&self) -> u8 {
        self.get_value()[0]
    }

    pub fn green(&self) -> u8 {
        self.get_value()[1]
    }

    pub fn blue(&self) -> u8 {
        self.get_value()[2]
    }
}
