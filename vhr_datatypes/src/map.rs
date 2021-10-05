use druid::im::Vector;
use druid::{Data, Lens};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::rc::Rc;
use std::collections::HashMap;

pub struct World {
    pub root_id: u64,
    pub entities: MapEntities,
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

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Entity {
    group_id: u64,
    entity_id: i32,
    // stuff: Vec<u8>,
    len: i32,
    owner_generation: i32,
    data_generation: i32,
    persistant: bool,
    owner_id: u64,
    timestamp: u64,
    package_version: i32,
    object_type: u8,
    is_distant: u8,
    prefab_id: i32,
    sector_x: i32,
    sector_y: i32,
    pos: Position,
    rotation: Quaternion,
    floats: HashMap<PropertyName, f32>,
    points: HashMap<PropertyName, Position>,
    rots: HashMap<PropertyName, Quaternion>,
    ints: HashMap<PropertyName, i32>,
    pairs: HashMap<PropertyName, (i32, i32)>,
    strings: HashMap<PropertyName, String>,
    // bytes: HashMap<PropertyName, Vec<u8>>,
}

#[derive(Hash, Default, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct PropertyName (u32);

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct EntityDeletionRecord {
    a: u64,
    b: i32,
    c: u64,
}
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    b: f32,
}

pub fn compact_vec_serialize<S, T>(vector: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    // serializer.serialize_u8(vector.len() as u8)?;
    let mut seq = serializer.serialize_seq(None)?;
    seq.serialize_element(&(vector.len() as u8))?;
    for e in vector {
        //     // serializer.serialize_any(e);
        seq.serialize_element(e)?;
        //     <T as Serialize>::serialize(e, serializer);
    }
    seq.end()
}
