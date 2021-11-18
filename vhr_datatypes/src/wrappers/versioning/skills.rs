use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::wrappers::versioning::{NoSuchVersion};

/// an enum with player skills versions
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum SkillsVersions {
    Zero(NoSuchVersion),
    One(Vec<(u32, f32)>),
    Two(Vec<Skill>),
}

impl SkillsVersions {
    pub fn new() -> Self {
        SkillsVersions::Two(Vec::new())
    }
    pub fn wrap_latest(inner: Vec<Skill>) -> SkillsVersions {
        SkillsVersions::Two(inner)
    }
    pub fn to_latest(self) -> Option<Vec<Skill>> {
        use SkillsVersions::*;
        match self {
            Zero(_) => None,
            One(_inner) => None, // todo: convert these.
            Two(inner) => Some(inner),
        }
    }
}
