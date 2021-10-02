use druid::im::Vector;
use druid::{Data, Lens};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::rc::Rc;

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
    pub deleted: DeletedEntities,
    pub loaded_chunks: Chunks,
    pub keys: Keys,
    pub structures: Structures,
    pub footer: MapFooter,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct MapHeader {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct MapEntities {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct DeletedEntities {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Chunks {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Keys {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Structures {}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct MapFooter {}
