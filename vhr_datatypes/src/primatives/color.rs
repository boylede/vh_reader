use serde::{Deserialize, Serialize};

/// color type, used in hair and beard colors.
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
    pub const BLACK: Color = Color {
        r: 0.3,
        g: 0.3,
        b: 0.3,
    };
    pub const BROWN: Color = Color {
        r: 0.4,
        g: 0.2,
        b: 0.2,
    };
    pub const DEFAULT: Color = Color {
        r: 141.0 / 255.0,
        g: 85.0 / 255.0,
        b: 36.0 / 255.0,
    };
}

/// a range which can be used to produce a color
/// as used in the game's gui
struct SkinRange(f32);

impl SkinRange {
    pub const MIN: Color = Color::BLACK;
    pub const MAX: Color = Color::WHITE;
    /// interpolates between the minimum and maximum colors
    /// for skin used in the game
    pub fn to_color(&self) -> Color {
        unimplemented!()
    }
}

// a pair of ranges, used to produce a color
// as used in the game's gui
struct HairRange(f32, f32);
impl HairRange {
    pub const MIN: Color = Color::WHITE;
    pub const MAX: Color = Color::BROWN;
    /// interpolates between the colors
    /// for hair used in the game
    pub fn to_color(self) -> Color {
        unimplemented!()
    }
}
