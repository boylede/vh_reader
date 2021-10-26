use super::KnownSize;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };
    pub const BROWN: Color = Color {
        r: 0.4,
        g: 0.2,
        b: 0.2,
    };
    // todo: figure out a good default
    pub const SKIN: Color = Color {
        r: 141.0 / 255.0,
        g: 85.0 / 255.0,
        b: 36.0 / 255.0,
    };
}

impl KnownSize for Color {
    fn count_bytes(&self) -> usize {
        12
    }
}

// pub struct ColorLens {
//     color: Channel,
// }

// impl ColorLens {
//     pub fn red() -> Self {
//         ColorLens { color: Channel::Red }
//     }
//     pub fn green() -> Self {
//         ColorLens { color: Channel::Green }
//     }
//     pub fn blue() -> Self {
//         ColorLens { color: Channel::Blue }
//     }
// }
/*
pub enum ColorLens {
    Red,
    Green,
    Blue,
}

impl Lens<Color, f64> for ColorLens {
    fn with<V, F: FnOnce(&f64) -> V>(&self, data: &Color, f: F) -> V {
        use ColorLens::*;
        let channel = match self {
            Red => data.r as f64,
            Green => data.g as f64,
            Blue => data.b as f64,
        };
        f(&channel)
    }
    fn with_mut<V, F: FnOnce(&mut f64) -> V>(&self, data: &mut Color, f: F) -> V {
        use ColorLens::*;
        let mut channel = match self {
            Red => data.r as f64,
            Green => data.g as f64,
            Blue => data.b as f64,
        };
        let value = f(&mut channel);
        match self {
            Red => data.r = channel as f32,
            Green => data.g = channel as f32,
            Blue => data.b = channel as f32,
        }
        value
    }
}

 */
