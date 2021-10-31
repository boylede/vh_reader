// for future use, not used currently, see separate implementation in mini_map.rs

use std::io::{Read, Write};
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use vhr_serde::de::{DeserializeOptions, VHDeserializer};
use vhr_serde::ser::{SerializeOptions, VHSerializer};

use flate2::{read::GzDecoder, write::GzEncoder, Compression};

use crate::common::*;


#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct CompressedWrapper<T> {
    inner: Vec<u8>,
    _phantom: PhantomData<T>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(from = "CompressedWrapper<T>", into = "CompressedWrapper<T>")]
pub struct CompressingWrapper<T: Clone> {
    inner: T,
}

impl<T: Clone> CompressingWrapper<T> {
    pub fn wrap(inner: T) -> Self {
        CompressingWrapper {
            inner,
        }
    }
}

impl<'de, T> From<CompressedWrapper<T>> for CompressingWrapper<T> where T: Deserialize<'de> + Clone {
    fn from(compressed: CompressedWrapper<T>) -> CompressingWrapper<T> {
        // println!("decompressing {} bytes", wrapper.inner.len());
        let mut buf = Vec::with_capacity(compressed.inner.len());
        let mut decoder = GzDecoder::new(compressed.inner.as_slice());
        decoder.read_to_end(&mut buf).unwrap();
        // println!("decompressed raw: {:?}", buf);
        // let option = SequenceLengthsAreSquared::compressed();
        let mut deserializer = VHDeserializer::from_owned(buf, ());
        let inner = <T as Deserialize>::deserialize(&mut deserializer).unwrap();
        CompressingWrapper {inner}
    }
}

impl<'de, T> From<CompressingWrapper<T>> for CompressedWrapper<T> where T: Serialize + Clone {
    fn from(uncompressed: CompressingWrapper<T>) -> CompressedWrapper<T> {
        let inner = uncompressed.inner;
        // let option = SequenceLengthsAreSquared::compressed();
        let mut serializer = VHSerializer::with_options(());
        <T as Serialize>::serialize(&inner, &mut serializer).unwrap();
        let uncompressed = serializer.to_inner();
        let mut inner = Vec::with_capacity(uncompressed.len());
        {
            let mut encoder = GzEncoder::new(&mut inner, Compression::best());
            encoder.write_all(&uncompressed).unwrap();
        }
        CompressedWrapper {
            inner,
            _phantom: PhantomData,
        }
    }
}

trait HasDeserializeOptions {
    type Options: DeserializeOptions;
    fn get_options() -> Self::Options;
}
