// unused currently

use std::io::{Read, Write};

use serde::{Deserialize, Serialize};
use vhr_serde::de::{DeserializeOptions, VHDeserializer};
use vhr_serde::ser::{SerializeOptions, VHSerializer};

use crate::common::*;

/// a wrapper to help construct images out of the mini map data
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "ImageWrapper", into = "ImageWrapper")]
pub struct SquareImage<Pixel: Clone> {
    edge_length: usize,
    inner: Vec<Pixel>,
}

/// the representation of the minimap data on disk
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageWrapper {
    inner: Vec<u8>,
}

impl<'de, T> From<ImageWrapper> for SquareImage<T>
where
    T: Clone,
{
    fn from(image: ImageWrapper) -> SquareImage<T> {
        unimplemented!()
    }
}

impl<'de, T> From<SquareImage<T>> for ImageWrapper
where
    T: Clone,
{
    fn from(image: SquareImage<T>) -> ImageWrapper {
        unimplemented!()
    }
}
