use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::rc::Rc;


use crate::prelude::*;
pub use character_data::CharacterData;
pub use mini_map::NewMiniMap;
pub use profile::Profile;
pub use version_enum::*;

pub use mini_map::NewMiniMapWrapper;
pub mod character_data;
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

impl<T> HashedBytes<T>
where
    T: KnownSize,
{
    pub fn from_inner(inner: T) -> Self {
        let content_size = <T as KnownSize>::count_bytes(&inner) as u32;
        HashedBytes {
            content_size,
            inner,
            hash: Vec::new(), // todo actual hash - probably have to wait until serialization
        }
    }
}

pub type CharacterFile = HashedBytes<PlayerProfile>;

trait CountSize {
    fn count_bytes(&self) -> usize;
}

// #[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
// pub struct Compendium {
//     pub recipes: Vec<String>,
//     pub craftbenches: Vec<(String, u32)>,
//     pub materials_list: Vec<String>,
//     pub places: Vec<String>,
//     pub unknown_list: Vec<String>,
//     pub trophies: Vec<String>,
//     pub biomes: Vec<BiomeId>,
//     pub tutorials: Vec<(String, String)>,
// }

// impl Compendium {
//     fn pre_serialize(&mut self) -> usize {
//         self.recipes.len()
//             + 4
//             + self
//                 .craftbenches
//                 .iter()
//                 .map(|c| c.0.len() + 1 + 4)
//                 .sum::<usize>()
//             + self
//                 .materials_list
//                 .iter()
//                 .map(|m| m.len() + 1)
//                 .sum::<usize>()
//             + self.places.iter().map(|m| m.len() + 1).sum::<usize>()
//             + self.unknown_list.iter().map(|m| m.len() + 1).sum::<usize>()
//             + self.trophies.iter().map(|m| m.len() + 1).sum::<usize>()
//             + (self.biomes.len() * 4)
//             + self
//                 .tutorials
//                 .iter()
//                 .map(|(a, b)| a.len() + b.len() + 2)
//                 .sum::<usize>()
//     }
// }

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Map {
    pub id: u64,
    pub spawn: MarkedPoint,
    pub position: MarkedPoint,
    pub death: MarkedPoint,
    pub home: Point,
    pub mini_map: Option<NewMiniMapWrapper>,
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

impl KnownSize for Map {
    fn count_bytes(&self) -> usize {
        <Option<NewMiniMapWrapper> as KnownSize>::count_bytes(&self.mini_map)
            + 8
            + 13
            + 13
            + 13
            + 12
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
