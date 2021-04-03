use crate::error::{Error, Result};
use serde::{de, Deserialize};

pub struct VHDeserializer<'de> {
    input: &'de [u8],
}

impl<'de> VHDeserializer<'de> {
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
