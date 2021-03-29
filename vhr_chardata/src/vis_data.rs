use druid::{Data, Lens};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::rc::Rc;

const MAP_WIDTH: usize = 2048;
const FOG_LENGTH: usize = MAP_WIDTH * MAP_WIDTH;

#[derive(PartialEq, Eq, Data, Clone, Lens, Debug, Deserialize, Serialize)]
pub struct VisibilityData {
    four: u32,
    twenty_fourty_eight: u32,
    fog: Rc<Vec<u8>>,
}

impl Default for VisibilityData {
    fn default() -> Self {
        VisibilityData {
            four: 4,
            twenty_fourty_eight: 2048,
            fog: Rc::new(vec![0u8;FOG_LENGTH]),
        }
    }
}
