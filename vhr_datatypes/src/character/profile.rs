use serde::{Deserialize, Serialize};

use super::{character_data::CharacterData, Map};
use crate::prelude::*;

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub kill_count: u32,
    pub death_count: u32,
    pub crafting_count: u32,
    pub building_count: u32,
    pub maps: Vec<Map>,
    pub name: String,
    pub player_id: u64,
    pub flag_a: u8,
    pub data: Option<CharacterData>,
}

impl KnownSize for Profile {
    fn count_bytes(&self) -> usize {
        4 + 4
            + 4
            + 4
            + 8
            + 1
            + <Vec<Map> as KnownSize>::count_bytes(&self.maps)
            + <String as KnownSize>::count_bytes(&self.name)
            + <Option<CharacterData> as KnownSize>::count_bytes(&self.data)
    }
}

// #[derive(PartialEq, Debug, Serialize, Deserialize)]
// pub struct Profile33 {
//     pub kill_count: u32,
//     pub death_count: u32,
//     pub crafting_count: u32,
//     pub building_count: u32,
//     pub maps: Vec<Map>,
//     pub name: String,
//     pub player_id: u64,
//     pub flag_a: u8,
//     pub data: Option<CharacterData>,
// }
