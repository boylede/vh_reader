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


    }
}

