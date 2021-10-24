use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::rc::Rc;

/// module for character files
pub mod character;
/// module for map files
pub mod map;
/// module for data types used throughout
pub mod common;
pub mod prelude;

pub use character::CharacterFile as FchFile;
pub use map::MapDatabaseFile as DbFile;
pub use map::WorldMetadataFile as FwlFile;
