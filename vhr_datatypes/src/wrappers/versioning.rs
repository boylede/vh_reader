use crate::prelude::*;
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

pub mod character;
pub mod inventory;
pub mod mini_map;
pub mod player;
pub mod skills;

/// an unknown data version
/// provides a stub so deserialization can fail
/// todo: write serializer to always fail.
#[derive(PartialEq, Debug, Serialize, Clone, Copy)]
pub struct UnknownVersion;

impl<'de> Deserialize<'de> for UnknownVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Err(D::Error::custom("This version is not known"))
    }
}

/// a version of the data structure known to not exist
/// e.g. 0 isnt used as a version as far as i can tell
/// todo: remove this boilerplate with changes to serialization so
/// these impossible versions don't need to be represented here
#[derive(PartialEq, Debug, Serialize, Clone, Copy)]
pub struct NoSuchVersion;

impl<'de> Deserialize<'de> for NoSuchVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Err(D::Error::custom("This type is not valid"))
    }
}
