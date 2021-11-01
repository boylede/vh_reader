use serde::{Deserialize, Serialize};

/// world-level events/ unlocks
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Keys {
    package_version: u32,
    keys_version: u32,
    keys: Vec<String>,
}
