use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use vhr_serde::{
    de::{DeserializeOptions, VHDeserializer},
    ser::VHSerializer,
};

use sha2::{Sha512, Digest};

use crate::prelude::*;
pub use character_data::CharacterData;
pub use mini_map::NewMiniMap;
pub use profile::Profile;
pub use version_enum::*;

pub use mini_map::NewMiniMapWrapper;
pub mod character_data;
mod mini_map;
mod profile;
mod version_enum;

/// A wrapper over a byte buffer that also contains a hash of the byte buffer
/// this is the representation found in the serialized output or deserialization input
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct HashedWrapper {
    inner: Vec<u8>,
    hash: Vec<u8>,
}

impl<'de, T> From<HashingWrapper<T>> for HashedWrapper
where
    T: Serialize + Clone,
{
    fn from(wrapper: HashingWrapper<T>) -> HashedWrapper {
        let inner = {
            let mut serializer = VHSerializer::new();
            <T as Serialize>::serialize(&wrapper.inner, &mut serializer).unwrap();
            serializer.to_inner()
        };
        let mut hasher = Sha512::new();
        hasher.update(&inner);
        let hash = hasher.finalize().to_vec();
        HashedWrapper {
            inner,
            hash,
        }
    }
}

impl<'de, T> From<HashedWrapper> for HashingWrapper<T>
where
    T: Deserialize<'de> + Clone,
{
    fn from(wrapper: HashedWrapper) -> HashingWrapper<T> {
        let mut hasher = Sha512::new();
        hasher.update(&wrapper.inner);
        let hash = hasher.finalize().to_vec();
        let hash_match = if hash == wrapper.hash {
            HashMatches::Match
        } else {
            HashMatches::Mismatch
        };

        let inner = {
            let mut deserializer = VHDeserializer::from_owned(wrapper.inner, ());
            <T as Deserialize>::deserialize(&mut deserializer).unwrap()
        };
        
        HashingWrapper {
            inner,
            hash: hash_match,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum HashMatches {
    /// indicates the hash matched input
    Match,
    /// indicates the hash did not match input
    Mismatch,
}

/// a container that wraps a serializable type and will provide the hash of that serialization 
/// after serializing. (De)Serialization converts to/from HashedWrapper
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(from = "HashedWrapper", into = "HashedWrapper")]
pub struct HashingWrapper<T: Clone> {
    // pub content_size: u32,
    pub inner: T,
    pub hash: HashMatches,
}

impl<T> HashingWrapper<T>
where
    T: Serialize + Clone,
{
    pub fn wrap(inner: T) -> HashedWrapper {
        HashingWrapper {
            inner,
            hash: HashMatches::Match,
        }
        .into()
    }
}

pub type CharacterFile = HashingWrapper<PlayerProfile>;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Map {
    pub id: u64,
    pub spawn: MarkedPoint,
    pub position: MarkedPoint,
    pub death: MarkedPoint,
    pub home: Point,
    pub mini_map: Option<NewMiniMapWrapper>,
}
