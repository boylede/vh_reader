use druid::im::Vector;
use druid::{Data, Lens};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;
use std::rc::Rc;

use properties::PropertyName;
mod properties;
const MAP_DATABASE_FILE_VERSION: i32 = 26;
const UNKNOWN_HEADER: i32 = 0;

pub struct World {
    pub root_id: u64,
    pub entities: MapEntities,
}

impl World {
    pub fn store(self) -> MapDatabaseFile {
        let mut header = MapHeader {
            version: MAP_DATABASE_FILE_VERSION,
            unknown: UNKNOWN_HEADER,
            start_time: 0.0,
            root_id: self.root_id,
        };

        unimplemented!()
    }
}

pub struct WorldMetadataFile {
    file_size: i32,
    map_version: i32,
    name: String,
    seed_name: String,
    seed: i32,
    uid: u64,
    world_gen_version: i32,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct MapDatabaseFile {
    pub header: MapHeader,
    pub entities: MapEntities,
    pub deleted: Vec<EntityDeletionRecord>,
    pub loaded_sectors: Vec<SectorCoordinate>,
    pub keys: Keys,
    pub structures: Option<Vec<Structure>>,
    pub footer: MapFooter,
}

impl MapDatabaseFile {
    pub fn load(self) -> World {
        World {
            root_id: self.header.root_id,
            entities: self.entities,
        }
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct MapHeader {
    version: i32,
    unknown: i32,
    start_time: f32,
    root_id: u64,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct MapEntities {
    pub next_id: i32,
    // count: i32,
    pub entities: Vec<Entity>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct SectorCoordinate {
    x: i32,
    y: i32,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Keys {
    package_version: i32,
    keys_version: i32,
    keys: Vec<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Structure {
    name: String,
    x: f32,
    y: f32,
    z: f32,
    seen: char,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct MapFooter {
    end_time: f32,
    padding: [u8; 17],
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Entity {
    pub group_id: u64,
    pub entity_id: i32,
    // stuff: Vec<u8>,
    pub len: i32,
    pub owner_generation: i32,
    pub data_generation: i32,
    pub persistant: bool,
    pub owner_id: u64,
    pub timestamp: u64,
    pub package_version: i32,
    pub object_type: u8,
    pub is_distant: u8,
    pub prefab_id: i32,
    pub sector_x: i32,
    pub sector_y: i32,
    pub pos: Position,
    pub rotation: Quaternion,
    pub floats: HashMap<PropertyName, f32>,
    pub points: HashMap<PropertyName, Position>,
    pub rots: HashMap<PropertyName, Quaternion>,
    pub ints: HashMap<PropertyName, i32>,
    pub pairs: HashMap<PropertyName, (i32, i32)>,
    pub strings: HashMap<PropertyName, String>,
    // pub bytes: HashMap<PropertyName, Vec<u8>>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct EntityDeletionRecord {
    a: u64,
    b: i32,
    c: u64,
}
#[derive(Default, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}
#[derive(Default, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    b: f32,
}
