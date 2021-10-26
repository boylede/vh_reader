use std::io::Read;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use vhr_serde::de::{VHDeserializer, DeserializeOptions};

use flate2::read::GzDecoder;
use crate::prelude::*;

use super::version_enum::*;
use crate::common::*;
// use crate::common::wrapper::{Wrapper, FromWrapper, WrapperArray};

// #[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
// pub struct MiniMap {
//     inner: Vec<u8>,
// }

// impl MiniMap {
//     pub fn len(&self) -> usize {
//         self.inner.len()
//     }
// }

pub type NewMiniMapWrapper = Wrapper<NewMiniMap>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NewMiniMap {
    Zero(NoSuchVersion), // invalid
    One(MiniMapOne),
    Two(MiniMapTwo),
    Three(MiniMapThree),
    Four(MiniMapFour),
    Five(MiniMapFive),
    Six(MiniMapSix),
    Seven(MiniMapSeven), // compressed
}

impl NewMiniMap {
    pub fn version(&self) -> usize {
        use NewMiniMap::*;
        match self {
            Zero(_) => 0, // invalid
            One(_) => 1,
            Two(_) => 2,
            Three(_) => 3,
            Four(_) => 4,
            Five(_) => 5,
            Six(_) => 6,
            Seven(_) => 7, // compressed
        }
    }
}


impl FromWrapper for NewMiniMap {
    fn from(wrapper: WrapperArray) -> Self {
        let version = {
            let mut deserializer = wrapper.get_vhrd(());
            let version = deserializer.peek_u32().unwrap();
            version
        };
        // let buffer = deserializer.into_inner();
        match version {
            1 => unimplemented!(),
            2 => unimplemented!(),
            3 => {
                let mut deserializer = wrapper.get_vhrd(SequenceLengthsAreSquared::first());
                <NewMiniMap as Deserialize>::deserialize(&mut deserializer).unwrap()
            },
            4 => {
                let mut deserializer = wrapper.get_vhrd(SequenceLengthsAreSquared::first());
                <NewMiniMap as Deserialize>::deserialize(&mut deserializer).unwrap()
            },
            5 => unimplemented!(),
            6 => unimplemented!(),
            7 => {
                let mut deserializer = wrapper.get_vhrd(SequenceLengthsAreSquared::second());
                <NewMiniMap as Deserialize>::deserialize(&mut deserializer).unwrap()
            },
            _ => panic!("mini map version not known"),
        }
        
        // unimplemented!()
    }
}

/// this is seriously a hack but i've painted myself into a corner and its too late to redesign so lets
/// just see where this goes....
struct SequenceLengthsAreSquared {
    enable: Vec<bool>,
}

impl std::fmt::Debug for SequenceLengthsAreSquared {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SequenceLengthsAreSquared").field("enable", &self.enable).finish()
    }
}

impl SequenceLengthsAreSquared {
    fn first() -> Self {
        SequenceLengthsAreSquared {
            enable: vec![true],
        }
    }
    fn second() -> Self {
        SequenceLengthsAreSquared {
            enable: vec![true,false],
        }
    }
}

impl DeserializeOptions for SequenceLengthsAreSquared {
    fn modify_sequence_length(&mut self, length: usize) -> usize {
        if let Some(true) = self.enable.pop() {
            println!("squaring {} = {}", length, length*length);
            // self.enable -= 1;
            length * length
        } else {
            length
        }

    }
}

impl KnownSize for NewMiniMap {
    fn count_bytes(&self) -> usize {
        use NewMiniMap::*;
        match self {
            Zero(_) => panic!("!"),
            One(inner) => unimplemented!(),
            Two(inner) => unimplemented!(),
            Three(inner) => unimplemented!(),
            Four(inner) => unimplemented!(),
            Five(inner) => unimplemented!(),
            Six(inner) => unimplemented!(),
            Seven(inner) => unimplemented!(),
        }
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
//     data: Vec<u8>,
// }

// #[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
// pub struct Poi {
//     name: String,
//     pos: Point,
//     kind: u8,
//     flags: u32,
// }

// impl Poi {
//     pub fn pre_serialize(&mut self) -> usize {
//         let size = self.name.len() + 1;
//         size + 8
//     }
// }
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MiniMapOne {
    data: Vec<u8>, // no size prepended here, texture_size ^2
}
impl std::fmt::Debug for MiniMapOne {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "map length: {}", self.data.len())
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MiniMapTwo {
    data: Vec<u8>, // no size prepended here, texture_size ^2
    pins: Vec<PinTwo>,
}

impl std::fmt::Debug for MiniMapTwo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "map length: {}", self.data.len())?;
        f.debug_list().entries(&self.pins).finish()
    }
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct PinTwo {
    text: String,
    pos: Point,
    kind: u32,
}
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MiniMapThree {
    data: Vec<u8>, // no size prepended here, texture_size ^2
    pins: Vec<PinThree>,
}

impl std::fmt::Debug for MiniMapThree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "map length: {}", self.data.len())?;
        f.debug_list().entries(&self.pins).finish()
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct PinThree {
    text: String,
    pos: Point,
    kind: u32,
    checked: bool,
}
#[derive(Clone, PartialEq,  Serialize, Deserialize)]
pub struct MiniMapFour {
    data: Vec<u8>, // no size prepended here, texture_size ^2
    pins: Vec<PinThree>,
    reference: bool,
}

// impl std::fmt::Debug for MiniMapFour {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("MiniMapFour").field("data", &self.data).field("pins", &self.pins).field("reference", &self.reference).finish()
//     }
// }

impl std::fmt::Debug for MiniMapFour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "map length: {}", self.data.len())?;
        f.debug_list().entries(&self.pins).finish()?;
        write!(f, "reference: {:?}", self.reference)
    }
}

// impl MiniMapFour {
//     pub fn from_reader(reader: &mut VHDeserializer) -> Self {
//         let data = {
//             let edge = reader.take_u32().unwrap() as usize;
//             let data = reader.take_byte_slice(edge*edge).unwrap();
//             Vec::from_parts(edge, data)
//         };
//         // pins: Vec<PinThree>,
//         // reference: bool,
//         unimplemented!()
//     }
// }

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct MiniMapFive {
    data: Vec<u8>, // no size prepended here, texture_size ^2
    others_explored: Box<[u8]>,
    pins: Vec<PinThree>,
    reference: bool,
}
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MiniMapSix {
    data: Vec<u8>, // no size prepended here, texture_size ^2
    others_explored: Box<[u8]>,
    pins: Vec<PinSix>,
    reference: bool,
}

impl std::fmt::Debug for MiniMapSix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "data: {} bytes not printed", self.data.len())?;
        writeln!(f, "others: {} bytes not printed", self.others_explored.len())?;
        f.debug_list().entries(&self.pins).finish()?;
        writeln!(f, "reference: {:?}", self.reference)
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct PinSix {
    text: String,
    pos: Point,
    kind: u32,
    checked: bool,
    owner: u64,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(from = "CompressedWrapper")]
pub struct MiniMapSeven {
    // compressed_len: usize,
    inner: MiniMapSix,
}



impl MiniMapSeven {
    pub fn from_six(inner: MiniMapSix) -> Self {
        MiniMapSeven {
            inner
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct CompressedWrapper {
    pub inner: Vec<u8>,
}

impl CompressedWrapper {
    pub fn get_vhrd<O>(&self, options: O) -> VHDeserializer<O> {
        VHDeserializer::from_bytes_options(&self.inner, options)
    }
}

// todo: can we make this generic? lots of errors when i tried first
impl<'de> From<CompressedWrapper> for MiniMapSeven  {
    fn from(wrapper: CompressedWrapper) -> MiniMapSeven {
        // println!("decompressing {} bytes", wrapper.inner.len());
        let mut buf = Vec::with_capacity(wrapper.inner.len());
        let mut decoder = GzDecoder::new(wrapper.inner.as_slice());
        decoder.read_to_end(&mut buf).unwrap();
        let mut deserializer = VHDeserializer::from_bytes_options(&buf, ());
        let minimap = <MiniMapSix as Deserialize>::deserialize(&mut deserializer).unwrap();
        // println!("minimap decompressed: {:?}", minimap);
        MiniMapSeven::from_six(minimap)
    }
}

