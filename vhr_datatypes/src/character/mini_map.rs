use std::io::Read;

use serde::{Deserialize, Serialize};
use vhr_serde::de::{VHDeserializer, DeserializeOptions};

use flate2::read::GzDecoder;

use super::version_enum::*;
use crate::common::*;

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

impl std::fmt::Debug for MiniMapFour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "map length: {}", self.data.len())?;
        f.debug_list().entries(&self.pins).finish()?;
        write!(f, "reference: {:?}", self.reference)
    }
}

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

