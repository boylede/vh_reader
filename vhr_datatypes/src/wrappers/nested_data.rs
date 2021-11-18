use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use std::collections::HashMap;

use crate::prelude::*;

#[derive(Default, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(from = "NestedData", into = "NestedData")]
pub struct NestingData(ByteBuf);

#[derive(Default, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct NestedData(ByteBuf);

impl<'de> From<NestingData> for NestedData
// where
//     T: Deserialize<'de> + Clone,
{
    fn from(data: NestingData) -> NestedData {
        let inner = data.0;
        println!("{:#?}\n\n\n\n\n", inner);
        NestedData(inner)
    }
}

impl<'de> From<NestedData> for NestingData
// where
//     T: Serialize + Clone,
{
    fn from(data: NestedData) -> NestingData {
        let inner = data.0;
        println!("{:#?}", inner);
        NestingData(inner)
    }
}
