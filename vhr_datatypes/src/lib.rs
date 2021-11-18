use prelude::*;
use serde::{Deserialize, Serialize};

pub mod prelude;

pub mod character;
pub mod entity;
pub mod food;
pub mod item;
pub mod key;
pub mod player;
pub mod primatives;
pub mod wrappers;

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
    pub unknown: i32,
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

/// the .fch character save file
pub type CharacterFile = HashingWrapper<CharacterFileVersions>;
