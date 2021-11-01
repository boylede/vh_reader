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
    pub initial_seed: String,
    pub data: Option<CharacterData>,
}
