use serde::{Deserialize, Serialize};

/// a three-dimensional point in the game world with a flag that indicates status.
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct MarkedPoint {
    pub mark: u8,
    pub point: Point,
}

/// a three-dimensional point in the game world
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// a rotation
#[derive(Default, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Quaternion {
    w: f32,
    x: f32,
    y: f32,
    z: f32,
}

/// the coordinates of a sector (chunk)
#[derive(Default, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct SectorCoordinate {
    pub x: i32,
    pub y: i32,
}
