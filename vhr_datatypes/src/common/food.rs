use super::KnownSize;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

pub type FoodBoth = Food16;

pub type FoodSingle = Food25;

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Food12 {
    pub name: String,
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
}

impl Food12 {
    pub fn upgrade(self) -> Food13 {
        let Food12 {
            name,
            a,
            b,
            c,
            d,
            e,
            f,
        } = self;
        Food13 {
            name,
            a,
            b,
            c,
            d,
            e,
            f,
            g: 1.0,
        }
    }
}

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Food13 {
    pub name: String,
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
    pub g: f32,
}

impl Food13 {
    pub fn upgrade(self) -> Food14 {
        let Food13 { name, .. } = self;
        Food14 { name, health: 50.0 } // todo: lookup table for values
    }
}

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Food14 {
    pub name: String,
    pub health: f32,
}

impl Food14 {
    pub fn upgrade(self) -> Food16 {
        let Food14 { name, health } = self;
        Food16 {
            name,
            health,
            stamina: 50.0,
        } // todo: lookup table for values
    }
}

pub type Food15 = Food14;
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Food16 {
    pub name: String,
    pub health: f32,
    pub stamina: f32,
}

impl Food16 {
    pub fn upgrade(self) -> Food25 {
        let Food16 {
            name,
            health,
            stamina,
        } = self;
        Food25 { name, time: 2000.0 } // todo: lookup table for times
    }
}

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Food25 {
    pub name: String,
    pub time: f32,
}

impl KnownSize for Food25 {
    fn count_bytes(&self) -> usize {
        <String as KnownSize>::count_bytes(&self.name) + 4
    }
}
