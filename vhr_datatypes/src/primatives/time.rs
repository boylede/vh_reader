use serde::{Deserialize, Serialize};

/// the tick on which an event occured
#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Tick(u64);
