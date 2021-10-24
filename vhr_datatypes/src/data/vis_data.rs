use serde::{Deserialize, Serialize, Serializer};
use std::rc::Rc;

const MAP_WIDTH: usize = 2048;
// const FOG_LENGTH: usize = MAP_WIDTH * MAP_WIDTH;

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub struct VisibilityData {
    four: u32,
    // twenty_fourty_eight: u32,
    fog: Rc<Vec<VisibilityRow>>,
}

impl VisibilityData {
    pub fn pre_serialize(&mut self) -> usize {
        // (*self.fog).len() * MAP_WIDTH + 4
        0x400008
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize)]
pub struct VisibilityRow {
    inner: Vec<u8>,
}

impl Default for VisibilityRow {
    fn default() -> Self {
        VisibilityRow {
            inner: vec![0; MAP_WIDTH],
        }
    }
}

impl Default for VisibilityData {
    fn default() -> Self {
        VisibilityData {
            four: 4,
            fog: Rc::new(vec![VisibilityRow::default(); MAP_WIDTH]),
        }
    }
}

impl Serialize for VisibilityRow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeTuple;
        let mut tup = serializer.serialize_tuple(self.inner.len())?;
        for byte in self.inner.iter() {
            tup.serialize_element(byte)?;
        }
        tup.end()
    }
}
