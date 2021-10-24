use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

#[derive(Default, Data, Clone, Lens, PartialEq, Debug, Serialize, Deserialize)]
pub struct MarkedPoint {
    pub mark: u8,
    pub point: Point,
}

#[derive(Default, Data, Clone, Lens, PartialEq, Debug, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
