use serde::{Deserialize, Serialize};

/// a bitfield indicating the biome(s)
/// each biome is a single bit set to 1
/// since the game sets it up this way
/// i am assuming there is somewhere this is used
/// with overlapping values, e.g. two biomes
/// at a biome border, but I have not found that yet.
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct BiomeId(u32);

impl BiomeId {
    pub const NONE: BiomeId = BiomeId(0);
    pub const MEADOWS: BiomeId = BiomeId(1);
    pub const SWAMP: BiomeId = BiomeId(2);
    pub const MOUNTAIN: BiomeId = BiomeId(4);
    pub const BLACKFOREST: BiomeId = BiomeId(8);
    pub const PLAINS: BiomeId = BiomeId(16);
    pub const ASHLANDS: BiomeId = BiomeId(32);
    pub const DEEPNOTH: BiomeId = BiomeId(64);
    pub const OCEAN: BiomeId = BiomeId(256);
    pub const MISTLANDS: BiomeId = BiomeId(512);
}

impl Default for BiomeId {
    fn default() -> Self {
        BiomeId::NONE
    }
}
