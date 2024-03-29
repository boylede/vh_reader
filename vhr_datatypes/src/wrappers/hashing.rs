use serde::{Deserialize, Serialize};
use vhr_serde::{
    de::{ VHDeserializer},
    ser::VHSerializer,
};

use sha2::{Digest, Sha512};

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
        HashedWrapper { inner, hash }
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
