use serde::{de, Deserialize};
use crate::error::{Error, Result};

pub struct VHDeserializer<'de> {
    // This string starts with the input data and characters are truncated off
    // the beginning as data is parsed.
    input: &'de [u8],
}

impl<'de> VHDeserializer<'de> {
    // By convention, `Deserializer` constructors are named like `from_xyz`.
    // That way basic use cases are satisfied by something like
    // `serde_json::from_str(...)` while advanced use cases that require a
    // deserializer can make one with `serde_json::Deserializer::from_str(...)`.
    pub fn from_bytes(input: &'de [u8]) -> Self {
        VHDeserializer { input }
    }
}

// pub fn from_bytes<'a, T>(s: &'a [u8]) -> Result<T>
// where
//     T: Deserialize<'a>,
// {
//     let mut deserializer = VHDeserializer::from_bytes(s);
//     let t = T::deserialize(&mut deserializer)?;
//     if deserializer.input.is_empty() {
//         Ok(t)
//     } else {
//         Err(Error::Other)
//     }
// }
