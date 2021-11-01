use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub mod inventory;
pub mod mini_map;
pub mod player;
pub mod character;
pub mod skills;

/// an unknown data version
/// provides a stub so deserialization can fail
/// todo: write (de)serializer to always fail.
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct UnknownVersion;

/// a version of the data structure known to not exist
/// e.g. 0 isnt used as a version as far as i can tell
/// todo: remove this boilerplate with changes to serialization so
/// these impossible versions don't need to be represented here
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct NoSuchVersion;
