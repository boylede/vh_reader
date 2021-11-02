use base64::{decode, encode};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use std::collections::HashMap;

use vhr_serde::de::{DeserializeOptions, VHDeserializer};
use vhr_serde::ser::{SerializeOptions, VHSerializer};

use crate::prelude::*;
use crate::wrappers::versioning::inventory::InventoryVersions;

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(from = "StringEncodedWrapper", into = "StringEncodedWrapper")]
pub enum StringEncodingWrapper {
    PlainString(String),
    Chest(Vec<Item>),
    Floats(Vec<f32>),
    Unknown(Vec<u8>),
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct StringEncodedWrapper(String);

impl<'de> From<StringEncodingWrapper> for StringEncodedWrapper
// where
//     T: Deserialize<'de> + Clone,
{
    fn from(data: StringEncodingWrapper) -> StringEncodedWrapper {
        use StringEncodingWrapper::*;
        let decoded = match data {
            PlainString(s) => {
                let mut serializer = VHSerializer::new();
                <String as Serialize>::serialize(&s, &mut serializer).unwrap();
                serializer.to_inner()
            },
            Chest(v)=> {
                let mut serializer = VHSerializer::new();
                let inventory = InventoryVersions::wrap_latest(v);
                <InventoryVersions as Serialize>::serialize(&inventory, &mut serializer).unwrap();
                serializer.to_inner()
            },
            Floats(f)=> {
                let mut serializer = VHSerializer::new();
                <Vec<f32> as Serialize>::serialize(&f, &mut serializer).unwrap();
                serializer.to_inner()
            },
            Unknown(b)=> b,
        };
        let inner = encode(decoded);
        StringEncodedWrapper(inner)
    }
}

impl<'de> From<StringEncodedWrapper> for StringEncodingWrapper
// where
//     T: Serialize + Clone,
{
    fn from(data: StringEncodedWrapper) -> StringEncodingWrapper {
        use StringEncodingWrapper::*;
        let inner = data.0;
        if inner.len() > 0 {
            if let Ok(decoded) = decode(&inner) {
                let mut deserializer = VHDeserializer::from_owned(decoded.clone(), ());
                if let Ok(inner) =
                    <InventoryVersions as Deserialize>::deserialize(&mut deserializer)
                {
                    let latest = inner.to_latest().unwrap();
                    // println!("found {} item inventory", latest.len());
                    Chest(latest)
                } else {
                    deserializer.rewind();
                    if let Ok(inner) = <Vec<f32> as Deserialize>::deserialize(&mut deserializer) {
                        // println!("found {} floats", inner.len());
                        Floats(inner)
                    } else {
                        let hex: String = decoded.iter().map(|b| format!("{:02x} ", b)).collect();
                        println!("decoded: [{}]", hex);
                        Unknown(decoded)
                    }
                }
            } else {
                // println!("couldnt decode: {}", inner);
                PlainString(inner)
            }
        } else {
            PlainString(inner)
        }
        
    }
}
