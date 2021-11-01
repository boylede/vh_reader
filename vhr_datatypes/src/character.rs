use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// character data
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Character {
    pub kill_count: u32,
    pub death_count: u32,
    pub crafting_count: u32,
    pub building_count: u32,
    pub maps: Vec<MapAtlas>,
    pub name: String,
    pub player_id: u64,
    pub initial_seed: String,
    pub data: Option<Wrapper<PlayerVersions>>,
}

/// Map atlas data
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct MapAtlas {
    pub id: u64,
    pub spawn: MarkedPoint,
    pub position: MarkedPoint,
    pub death: MarkedPoint,
    pub home: Point,
    pub mini_map: Option<MiniMap>,
}
