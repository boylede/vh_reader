use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use vhr_serde::de::{DeserializeOptions, VHDeserializer};
use vhr_serde::ser::{SerializeOptions, VHSerializer};

use crate::prelude::*;
use crate::wrappers::versioning::{NoSuchVersion, UnknownVersion};

pub type MiniMap = Wrapper<MiniMapVersions>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MiniMapVersions {
    Zero(NoSuchVersion),
    One(MiniMapOne),
    Two(MiniMapTwo),
    Three(MiniMapThree),
    Four(MiniMapFour),
    Five(MiniMapFive),
    Six(MiniMapSix),
    Seven(MiniMapSeven),
}

impl MiniMapVersions {
    pub fn version(&self) -> usize {
        use MiniMapVersions::*;
        match self {
            Zero(_) => 0,
            One(_) => 1,
            Two(_) => 2,
            Three(_) => 3,
            Four(_) => 4,
            Five(_) => 5,
            Six(_) => 6,
            Seven(_) => 7,
        }
    }
    pub fn into_latest(self) -> MiniMapSeven {
        use MiniMapVersions::*;
        match self {
            Zero(_) => panic!("invalid map version"),
            One(inner) => inner
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade(),
            Two(inner) => inner.upgrade().upgrade().upgrade().upgrade().upgrade(),
            Three(inner) => inner.upgrade().upgrade().upgrade().upgrade(),
            Four(inner) => inner.upgrade().upgrade().upgrade(),
            Five(inner) => inner.upgrade().upgrade(),
            Six(inner) => inner.upgrade(),
            Seven(inner) => inner,
        }
    }
}

impl Wrapped for MiniMapVersions {
    fn strip(wrapper: WrapperArray) -> Wrapper<Self> {
        // todo: this is only valid for some versions of the map
        let option = SequenceLengthsAreSquared::uncompressed();
        let mut deserializer = VHDeserializer::from_owned(wrapper.inner, option);
        let inner = <Self as Deserialize>::deserialize(&mut deserializer).unwrap();
        Wrapper { inner }
    }
    fn wrap(item: Wrapper<Self>) -> WrapperArray {
        let inner = {
            // todo: this is only valid for some versions of the map
            let option = SequenceLengthsAreSquared::uncompressed();
            let mut serializer = VHSerializer::with_options(option);
            <Self as Serialize>::serialize(&item.inner, &mut serializer).unwrap();
            serializer.to_inner()
        };
        WrapperArray { inner }
    }
}

/// this is seriously a hack but i've painted myself into a corner and its too late to redesign so lets
/// just see where this goes....
struct SequenceLengthsAreSquared {
    enable: Vec<Modifier>,
    saved: usize,
}

#[derive(Debug)]
enum Modifier {
    Pass,
    Modify,
    ModifyAndSave,
    Recall,
}

impl std::fmt::Debug for SequenceLengthsAreSquared {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SequenceLengthsAreSquared")
            .field("enable", &self.enable)
            .finish()
    }
}

impl SequenceLengthsAreSquared {
    /// sequence lengths in early versions: the first one is squared, then normal after that
    fn early() -> Self {
        use Modifier::*;
        SequenceLengthsAreSquared {
            enable: vec![Modify],
            saved: 0,
        }
    }
    /// seq lengths in pre-compressed versions: normal, then squared, then normal
    fn uncompressed() -> Self {
        use Modifier::*;
        SequenceLengthsAreSquared {
            enable: vec![Modify, Pass],
            saved: 0,
        }
    }
    // sequence lengths in compressed versions, squared, then skipped, then normal
    fn compressed() -> Self {
        use Modifier::*;
        SequenceLengthsAreSquared {
            enable: vec![Recall, ModifyAndSave],
            saved: 0,
        }
    }
}

impl DeserializeOptions for SequenceLengthsAreSquared {
    fn omit_sequence_length(&mut self) -> bool {
        use Modifier::*;
        match self.enable.iter().last() {
            Some(Pass) => false,
            Some(Modify) => false,
            Some(ModifyAndSave) => false,
            Some(Recall) => true,
            None => false,
        }
    }
    fn modify_sequence_length(&mut self, length: usize) -> usize {
        use Modifier::*;
        match self.enable.pop() {
            Some(Pass) => length,
            Some(Modify) => length * length,
            Some(ModifyAndSave) => {
                println!("saving value {} as {}", length, length * length);
                self.saved = length * length;
                length * length
            }
            Some(Recall) => {
                println!("recalling value {}", self.saved);
                self.saved
            }
            None => length,
        }
    }
}

impl SerializeOptions for SequenceLengthsAreSquared {
    fn modify_sequence_length(&mut self, length: usize) -> Option<usize> {
        use Modifier::*;
        match self.enable.pop() {
            Some(Pass) => Some(length),
            Some(Modify) => Some(f32::sqrt(length as f32).floor() as usize),
            Some(ModifyAndSave) => {
                let sqrt = f32::sqrt(length as f32).floor() as usize;
                // println!("saving value {} as {}", length, sqrt);
                self.saved = sqrt;
                Some(sqrt)
            }
            Some(Recall) => {
                // println!("recalling value {}", self.saved);
                let sqrt = f32::sqrt(length as f32).floor() as usize;
                // println!("expected {}", sqrt);
                if sqrt != self.saved {
                    panic!(
                        "tried to serialize a sequence using an incorrect size, saved previously."
                    );
                } else {
                    None
                }
            }
            None => Some(length),
        }
    }
    fn omit_sequence_length(&mut self) -> bool {
        use Modifier::*;
        match self.enable.pop() {
            Some(Pass) => false,
            Some(Modify) => false,
            Some(ModifyAndSave) => false,
            Some(Recall) => true,
            None => false,
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MiniMapOne {
    data: Vec<u8>,
}

impl MiniMapOne {
    pub fn upgrade(self) -> MiniMapTwo {
        let MiniMapOne { data } = self;
        MiniMapTwo {
            data,
            pins: Vec::new(),
        }
    }
}

impl std::fmt::Debug for MiniMapOne {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "map length: {}", self.data.len())
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MiniMapTwo {
    data: Vec<u8>,
    pins: Vec<PinTwo>,
}

impl MiniMapTwo {
    pub fn upgrade(self) -> MiniMapThree {
        let MiniMapTwo { data, pins } = self;
        MiniMapThree {
            data,
            pins: pins.into_iter().map(|p| p.upgrade()).collect(),
        }
    }
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

impl PinTwo {
    pub fn upgrade(self) -> PinThree {
        let PinTwo { text, pos, kind } = self;
        PinThree {
            text,
            pos,
            kind,
            checked: false,
        }
    }
}
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MiniMapThree {
    data: Vec<u8>,
    pins: Vec<PinThree>,
}

impl MiniMapThree {
    pub fn upgrade(self) -> MiniMapFour {
        let MiniMapThree { data, pins } = self;
        MiniMapFour {
            data,
            pins,
            reference: false,
        }
    }
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

impl PinThree {
    pub fn upgrade(self) -> PinSix {
        let PinThree {
            text,
            pos,
            kind,
            checked,
        } = self;
        PinSix {
            text,
            pos,
            kind,
            checked,
            owner: 0,
        }
    }
}
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MiniMapFour {
    data: Vec<u8>,
    pins: Vec<PinThree>,
    reference: bool,
}

impl MiniMapFour {
    pub fn upgrade(self) -> MiniMapFive {
        let MiniMapFour {
            data,
            pins,
            reference,
        } = self;
        let others_explored = vec![0u8; data.len()];
        MiniMapFive {
            data,
            others_explored,
            pins,
            reference,
        }
    }
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
    data: Vec<u8>,
    others_explored: Vec<u8>,
    pins: Vec<PinThree>,
    reference: bool,
}

impl MiniMapFive {
    pub fn upgrade(self) -> MiniMapSix {
        let MiniMapFive {
            data,
            others_explored,
            pins,
            reference,
        } = self;
        MiniMapSix {
            data,
            others_explored,
            pins: pins.into_iter().map(|p| p.upgrade()).collect(),
            reference,
        }
    }
}
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MiniMapSix {
    pub data: Vec<u8>,
    pub others_explored: Vec<u8>,
    pub pins: Vec<PinSix>,
    pub reference: bool,
}

impl MiniMapSix {
    pub fn upgrade(self) -> MiniMapSeven {
        let inner = self;
        MiniMapSeven { inner }
    }
}

impl std::fmt::Debug for MiniMapSix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "data: {} bytes not printed", self.data.len())?;
        writeln!(
            f,
            "others: {} bytes not printed",
            self.others_explored.len()
        )?;
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
#[serde(from = "CompressedWrapper", into = "CompressedWrapper")]
pub struct MiniMapSeven {
    pub inner: MiniMapSix,
}

impl MiniMapSeven {
    pub fn from_six(inner: MiniMapSix) -> Self {
        MiniMapSeven { inner }
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
impl<'de> From<CompressedWrapper> for MiniMapSeven {
    fn from(wrapper: CompressedWrapper) -> MiniMapSeven {
        // println!("decompressing {} bytes", wrapper.inner.len());
        let mut buf = Vec::with_capacity(wrapper.inner.len());
        let mut decoder = GzDecoder::new(wrapper.inner.as_slice());
        decoder.read_to_end(&mut buf).unwrap();
        // println!("minimap decompressed raw: {:?}", buf);
        let option = SequenceLengthsAreSquared::compressed();
        let mut deserializer = VHDeserializer::from_bytes_options(&buf, option);
        let minimap = <MiniMapSix as Deserialize>::deserialize(&mut deserializer).unwrap();
        // println!("minimap decompressed: {:?}", minimap);
        MiniMapSeven::from_six(minimap)
    }
}

impl<'de> From<MiniMapSeven> for CompressedWrapper {
    fn from(wrapper: MiniMapSeven) -> CompressedWrapper {
        let six = wrapper.inner;
        let option = SequenceLengthsAreSquared::compressed();
        let mut serializer = VHSerializer::with_options(option);
        <MiniMapSix as Serialize>::serialize(&six, &mut serializer).unwrap();
        let uncompressed = serializer.to_inner();
        // println!("minimap to-compress: {:?}", uncompressed);
        let mut compressed_buffer = Vec::with_capacity(uncompressed.len());
        {
            let mut encoder = GzEncoder::new(&mut compressed_buffer, Compression::new(2));
            encoder.write_all(&uncompressed).unwrap();
        }
        CompressedWrapper {
            inner: compressed_buffer,
        }
    }
}
