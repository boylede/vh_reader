use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::rc::Rc;

use crate::prelude::*;

use super::version_enum::*;

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct MiniMap {
    inner: Vec<u8>,
}

impl MiniMap {
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

// #[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
// pub struct MiniMap {
//     pub size: u32,
//     pub inner: VersionedMiniMap,
// }

// #[derive(Default, Clone, Lens, PartialEq, Debug, Serialize, Deserialize)]
// /// the compressed mini map
// /// todo: add 7zip (de) compression to (de)serialize
// pub struct CompressedMiniMap {
//     data: Vec<u8>,
// }

// #[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
// pub struct UncompressedMiniMap {
//     data: SquareVec<u8>,
// }

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Poi {
    name: String,
    pos: Point,
    kind: u8,
    flags: u32,
}

impl Poi {
    pub fn pre_serialize(&mut self) -> usize {
        let size = self.name.len() + 1;
        size + 8
    }
}
