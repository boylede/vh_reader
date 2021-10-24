use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

use super::Map;
use crate::prelude::*;

#[derive(Default, Data, Clone, Lens, PartialEq, Debug, Serialize, Deserialize)]
pub struct FoodBoth {
    name: String,
    health: f32,
    stamina: f32,
}

impl FoodBoth {
    fn pre_serialize(&mut self) -> usize {
        self.name.len() + 1 + 8
    }
}

#[derive(Default, Data, Clone, Lens, PartialEq, Debug, Serialize, Deserialize)]
pub struct FoodSingle {
    name: String,
    value: f32,
}
