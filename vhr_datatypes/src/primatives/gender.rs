use crate::prelude::*;
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Player character gender, controls the mesh model used.
#[derive(PartialEq, Eq, Clone, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
#[non_exhaustive]
pub enum Gender {
    Female = 1,
    Male = 0,
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Male
    }
}
