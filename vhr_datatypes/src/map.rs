use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use std::collections::HashMap;

use hashed_string::HashedString;
use crate::common::*;
pub mod hashed_string;

/// the map metadata file
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct WorldMetadataFile {
    file_size: i32,
    map_version: i32,
    name: String,
    seed_name: String,
    seed: i32,
    uid: u64,
    world_gen_version: i32,
}

/// the .db file used to store map info
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct MapDatabaseFile {
    pub version: u32,
    pub unknown: i32, // shouldnt exist?
    pub start_time: f32,
    pub root_id: u64,
    pub next_id: u32,
    pub entities: Vec<Entity>,
    pub deleted: Vec<EntityDeletionRecord>,
    pub loaded_sectors: Vec<SectorCoordinate>,
    pub keys: Keys,
    pub structures: Option<Vec<Structure>>,
    pub end_time: f32,
    pub padding: [u8; 17],
}

/// the coordinates of a sector (chunk)
#[derive(Default, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct SectorCoordinate {
    pub x: i32,
    pub y: i32,
}

/// world-level events/ unlocks
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Keys {
    package_version: u32,
    keys_version: u32,
    keys: Vec<String>,
}

/// something that appears in the map
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Structure {
    name: String,
    pos: Point,
    seen: char,
}

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
    pub strings: HashMap<HashedString, String>,
    pub bytes: HashMap<HashedString, ByteBuf>,
}

/// a record of when an entity was removed from the world
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct EntityDeletionRecord {
    user_id: u64,
    entity_id: i32,
    deleted: Tick,
}

/// the tick on which an event occured
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Tick(u64);

/// a rotation
#[derive(Default, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Quaternion {
    w: f32,
    x: f32,
    y: f32,
    z: f32,
}
