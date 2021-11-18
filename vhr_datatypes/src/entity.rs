use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::prelude::*;

/// an entity in the map
#[derive(Default, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Entity {
    pub user_id: u64,
    pub entity_id: i32,
    pub len: i32,
    pub owner_generation: i32,
    pub data_generation: i32,
    pub persistant: bool,
    pub owner_id: u64,
    pub timestamp: u64,
    pub package_version: i32,
    pub object_type: u8,
    pub is_distant: u8,
    pub prefab_id: HashedString,
    pub sector: SectorCoordinate,
    pub pos: Point,
    pub rotation: Quaternion,
    pub floats: HashMap<HashedString, f32>,
    pub points: HashMap<HashedString, Point>,
    pub rots: HashMap<HashedString, Quaternion>,
    pub ints: HashMap<HashedString, i32>,
    pub pairs: HashMap<HashedString, (i32, i32)>,
    pub strings: HashMap<HashedString, StringEncodingWrapper>,
    pub bytes: HashMap<HashedString, NestedData>,
}

/// a record of when an entity was removed from the world
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct EntityDeletionRecord {
    user_id: u64,
    entity_id: i32,
    deleted: Tick,
}

/// something that appears in the map
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Structure {
    name: String,
    pos: Point,
    seen: char,
}
