use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::rc::Rc;

use crate::prelude::*;

pub use mini_map::MiniMap;
pub use version_enum::*;

pub mod character_data;
mod food;
mod inventory;
mod mini_map;
mod profile;
mod version_enum;

/// a container that hashes its content and lists the size of the content up front
/// todo: implement serialize to calculate the hash, and deserialize to notice bad hashes (but not fail because who cares)
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct HashedBytes<T> {
    pub content_size: u32,
    pub inner: T,
    pub hash: Vec<u8>,
}

pub type CharacterFile = HashedBytes<PlayerProfile>;

trait CountSize {
    fn count_bytes(&self) -> usize;
}

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Compendium {
    pub recipes: Vec<String>,
    pub craftbenches: Vec<(String, u32)>,
    pub materials_list: Vec<String>,
    pub places: Vec<String>,
    pub unknown_list: Vec<String>,
    pub trophies: Vec<String>,
    pub biomes: Vec<BiomeId>,
    pub tutorials: Vec<(String, String)>,
}

impl Compendium {
    fn pre_serialize(&mut self) -> usize {
        self.recipes.len()
            + 4
            + self
                .craftbenches
                .iter()
                .map(|c| c.0.len() + 1 + 4)
                .sum::<usize>()
            + self
                .materials_list
                .iter()
                .map(|m| m.len() + 1)
                .sum::<usize>()
            + self.places.iter().map(|m| m.len() + 1).sum::<usize>()
            + self.unknown_list.iter().map(|m| m.len() + 1).sum::<usize>()
            + self.trophies.iter().map(|m| m.len() + 1).sum::<usize>()
            + (self.biomes.len() * 4)
            + self
                .tutorials
                .iter()
                .map(|(a, b)| a.len() + b.len() + 2)
                .sum::<usize>()
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Map {
    pub id: u64,
    pub spawn: MarkedPoint,
    pub position: MarkedPoint,
    pub death: MarkedPoint,
    pub home: Point,
    pub mini_map: Option<MiniMap>,
}

impl Map {
    pub fn pre_serialize(&mut self) -> usize {
        unimplemented!()
        // let mut size: usize = self.mini_map.compressed.len() + 4;
        // //self.fog_of_war.pre_serialize();
        // // size += self
        // //     .pois
        // //     .iter_mut()
        // //     .map(|p| p.pre_serialize())
        // //     .sum::<usize>()
        // //     + 4;
        // // if self.extra.is_some() {
        // //     size += 1;
        // // }

        // self.map_size = size as u32;
        // size + 24
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
#[non_exhaustive]
pub enum Gender {
    Female = 1,
    Male = 0,
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Male
    }
}

#[derive(PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum BeardType {
    Beard1,
    Beard2,
}
