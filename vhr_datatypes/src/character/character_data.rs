use serde::{Deserialize, Serialize};

use crate::prelude::*;

use super::version_enum::*;
use crate::common::food::*;
use crate::common::hair::{BeardStyle, HairStyle};

pub type CharacterData = Wrapper<Player>;

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerOne {
    pub current_health: f32,
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub unknown: Vec<String>,
    pub materials: Vec<String>,
    pub tutorials: Vec<String>,
}

impl PlayerOne {
    #[inline]
    pub fn upgrade(self) -> PlayerTwo {
        let PlayerOne {
            current_health,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
        } = self;
        PlayerTwo {
            current_health,
            zdoid: (0, 0),
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerTwo {
    pub current_health: f32,
    pub zdoid: (u64, u32),
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub unknown: Vec<String>,
    pub materials: Vec<String>,
    pub tutorials: Vec<String>,
}

impl PlayerTwo {
    #[inline]
    pub fn upgrade(self) -> PlayerThree {
        let PlayerTwo {
            current_health,
            zdoid,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
        } = self;
        let _ = zdoid;
        PlayerThree {
            current_health,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerThree {
    pub current_health: f32,
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub unknown: Vec<String>,
    pub materials: Vec<String>,
    pub tutorials: Vec<String>,
}

impl PlayerThree {
    #[inline]
    pub fn upgrade(self) -> PlayerFour {
        let PlayerThree {
            current_health,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
        } = self;
        PlayerFour {
            current_health,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
            beard_type: BeardStyle::beard_1(),
            hair_type: HairStyle::hair_1(),
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerFour {
    pub current_health: f32,
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub unknown: Vec<String>,
    pub materials: Vec<String>,
    pub tutorials: Vec<String>,
    pub beard_type: BeardStyle,
    pub hair_type: HairStyle,
}

impl PlayerFour {
    #[inline]
    pub fn upgrade(self) -> PlayerFive {
        let PlayerFour {
            current_health,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
            beard_type,
            hair_type,
        } = self;
        PlayerFive {
            current_health,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
            beard_type,
            hair_type,
            skin: Color::DEFAULT,
            hair: Color::BROWN,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerFive {
    pub current_health: f32,
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub unknown: Vec<String>,
    pub materials: Vec<String>,
    pub tutorials: Vec<String>,
    pub beard_type: BeardStyle,
    pub hair_type: HairStyle,
    pub skin: Color,
    pub hair: Color,
}

impl PlayerFive {
    #[inline]
    pub fn upgrade(self) -> PlayerSix {
        let PlayerFive {
            current_health,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
            beard_type,
            hair_type,
            skin,
            hair,
        } = self;
        PlayerSix {
            current_health,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
            uniques: Vec::new(),
            beard_type,
            hair_type,
            skin,
            hair,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerSix {
    pub current_health: f32,
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub unknown: Vec<String>,
    pub materials: Vec<String>,
    pub tutorials: Vec<String>,
    pub uniques: Vec<String>,
    pub beard_type: BeardStyle,
    pub hair_type: HairStyle,
    pub skin: Color,
    pub hair: Color,
}

impl PlayerSix {
    #[inline]
    pub fn upgrade(self) -> PlayerSeven {
        let PlayerSix {
            current_health,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
            uniques,
            beard_type,
            hair_type,
            skin,
            hair,
        } = self;
        PlayerSeven {
            max_health: 100.0,
            current_health,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
            uniques,
            beard_type,
            hair_type,
            skin,
            hair,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerSeven {
    pub max_health: f32,
    pub current_health: f32,
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub unknown: Vec<String>,
    pub materials: Vec<String>,
    pub tutorials: Vec<String>,
    pub uniques: Vec<String>,
    pub beard_type: BeardStyle,
    pub hair_type: HairStyle,
    pub skin: Color,
    pub hair: Color,
}

impl PlayerSeven {
    #[inline]
    pub fn upgrade(self) -> PlayerEight {
        let PlayerSeven {
            max_health,
            current_health,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
            uniques,
            beard_type,
            hair_type,
            skin,
            hair,
        } = self;
        PlayerEight {
            max_health,
            current_health,
            first_spawn: 0,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
            uniques,
            beard_type,
            hair_type,
            skin,
            hair,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerEight {
    pub max_health: f32,
    pub current_health: f32,
    pub first_spawn: u8,
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub unknown: Vec<String>,
    pub materials: Vec<String>,
    pub tutorials: Vec<String>,
    pub uniques: Vec<String>,
    pub beard_type: BeardStyle,
    pub hair_type: HairStyle,
    pub skin: Color,
    pub hair: Color,
}

impl PlayerEight {
    #[inline]
    pub fn upgrade(self) -> PlayerNine {
        let PlayerEight {
            max_health,
            current_health,
            first_spawn,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
            uniques,
            beard_type,
            hair_type,
            skin,
            hair,
        } = self;
        PlayerNine {
            max_health,
            current_health,
            first_spawn,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
            uniques,
            trophies: Vec::new(),
            beard_type,
            hair_type,
            skin,
            hair,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerNine {
    pub max_health: f32,
    pub current_health: f32,
    pub first_spawn: u8,
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub unknown: Vec<String>,
    pub materials: Vec<String>,
    pub tutorials: Vec<String>,
    pub uniques: Vec<String>,
    pub trophies: Vec<String>,
    pub beard_type: BeardStyle,
    pub hair_type: HairStyle,
    pub skin: Color,
    pub hair: Color,
}

impl PlayerNine {
    #[inline]
    pub fn upgrade(self) -> PlayerTen {
        let PlayerNine {
            max_health,
            current_health,
            first_spawn,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
            uniques,
            trophies,
            beard_type,
            hair_type,
            skin,
            hair,
        } = self;
        PlayerTen {
            max_health,
            current_health,
            stamina: 0.0,
            first_spawn,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
            uniques,
            trophies,
            beard_type,
            hair_type,
            skin,
            hair,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerTen {
    pub max_health: f32,
    pub current_health: f32,
    pub stamina: f32,
    pub first_spawn: u8,
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub unknown: Vec<String>,
    pub materials: Vec<String>,
    pub tutorials: Vec<String>,
    pub uniques: Vec<String>,
    pub trophies: Vec<String>,
    pub beard_type: BeardStyle,
    pub hair_type: HairStyle,
    pub skin: Color,
    pub hair: Color,
}

impl PlayerTen {
    #[inline]
    pub fn upgrade(self) -> PlayerEleven {
        let PlayerTen {
            max_health,
            current_health,
            stamina,
            first_spawn,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
            uniques,
            trophies,
            beard_type,
            hair_type,
            skin,
            hair,
        } = self;
        PlayerEleven {
            max_health,
            current_health,
            stamina,
            first_spawn,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
            uniques,
            trophies,
            beard_type,
            hair_type,
            skin,
            hair,
            gender: Gender::Male,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerEleven {
    pub max_health: f32,
    pub current_health: f32,
    pub stamina: f32,
    pub first_spawn: u8,
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub unknown: Vec<String>,
    pub materials: Vec<String>,
    pub tutorials: Vec<String>,
    pub uniques: Vec<String>,
    pub trophies: Vec<String>,
    pub beard_type: BeardStyle,
    pub hair_type: HairStyle,
    pub skin: Color,
    pub hair: Color,
    pub gender: Gender,
}

impl PlayerEleven {
    #[inline]
    pub fn upgrade(self) -> PlayerTwelve {
        let PlayerEleven {
            max_health,
            current_health,
            stamina,
            first_spawn,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
            uniques,
            trophies,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
        } = self;
        PlayerTwelve {
            max_health,
            current_health,
            stamina,
            first_spawn,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
            uniques,
            trophies,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food: Vec::new(),
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerTwelve {
    pub max_health: f32,
    pub current_health: f32,
    pub stamina: f32,
    pub first_spawn: u8,
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub unknown: Vec<String>,
    pub materials: Vec<String>,
    pub tutorials: Vec<String>,
    pub uniques: Vec<String>,
    pub trophies: Vec<String>,
    pub beard_type: BeardStyle,
    pub hair_type: HairStyle,
    pub skin: Color,
    pub hair: Color,
    pub gender: Gender,
    pub food: Vec<Food12>,
}

impl PlayerTwelve {
    #[inline]
    pub fn upgrade(self) -> PlayerThirteen {
        let PlayerTwelve {
            max_health,
            current_health,
            stamina,
            first_spawn,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
            uniques,
            trophies,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food,
        } = self;
        PlayerThirteen {
            max_health,
            current_health,
            stamina,
            first_spawn,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
            uniques,
            trophies,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food: food.into_iter().map(|f| f.upgrade()).collect(),
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerThirteen {
    pub max_health: f32,
    pub current_health: f32,
    pub stamina: f32,
    pub first_spawn: u8,
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub unknown: Vec<String>,
    pub materials: Vec<String>,
    pub tutorials: Vec<String>,
    pub uniques: Vec<String>,
    pub trophies: Vec<String>,
    pub beard_type: BeardStyle,
    pub hair_type: HairStyle,
    pub skin: Color,
    pub hair: Color,
    pub gender: Gender,
    pub food: Vec<Food13>,
}

impl PlayerThirteen {
    #[inline]
    pub fn upgrade(self) -> PlayerFourteen {
        let PlayerThirteen {
            max_health,
            current_health,
            stamina,
            first_spawn,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
            uniques,
            trophies,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food,
        } = self;
        let _ = unknown;
        PlayerFourteen {
            max_health,
            current_health,
            stamina,
            first_spawn,
            inventory,
            recipes,
            unknown: Vec::new(),
            materials,
            tutorials,
            uniques,
            trophies,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food: food.into_iter().map(|f| f.upgrade()).collect(),
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerFourteen {
    pub max_health: f32,
    pub current_health: f32,
    pub stamina: f32,
    pub first_spawn: u8,
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub unknown: Vec<String>,
    pub materials: Vec<String>,
    pub tutorials: Vec<String>,
    pub uniques: Vec<String>,
    pub trophies: Vec<String>,
    pub beard_type: BeardStyle,
    pub hair_type: HairStyle,
    pub skin: Color,
    pub hair: Color,
    pub gender: Gender,
    pub food: Vec<Food14>,
}

impl PlayerFourteen {
    #[inline]
    pub fn upgrade(self) -> PlayerFifteen {
        let PlayerFourteen {
            max_health,
            current_health,
            stamina,
            first_spawn,
            inventory,
            recipes,
            unknown,
            materials,
            tutorials,
            uniques,
            trophies,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food,
        } = self;
        let _ = unknown;
        PlayerFifteen {
            max_health,
            current_health,
            stamina,
            first_spawn,
            inventory,
            recipes,
            known_stations: Vec::new(),
            materials,
            tutorials,
            uniques,
            trophies,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerFifteen {
    pub max_health: f32,
    pub current_health: f32,
    pub stamina: f32,
    pub first_spawn: u8,
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub known_stations: Vec<(String, u32)>,
    pub materials: Vec<String>,
    pub tutorials: Vec<String>,
    pub uniques: Vec<String>,
    pub trophies: Vec<String>,
    pub beard_type: BeardStyle,
    pub hair_type: HairStyle,
    pub skin: Color,
    pub hair: Color,
    pub gender: Gender,
    pub food: Vec<Food15>,
}

impl PlayerFifteen {
    #[inline]
    pub fn upgrade(self) -> PlayerSixteen {
        let PlayerFifteen {
            max_health,
            current_health,
            stamina,
            first_spawn,
            inventory,
            recipes,
            known_stations,
            materials,
            tutorials,
            uniques,
            trophies,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food,
        } = self;
        PlayerSixteen {
            max_health,
            current_health,
            stamina,
            first_spawn,
            inventory,
            recipes,
            known_stations,
            materials,
            tutorials,
            uniques,
            trophies,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food: food.into_iter().map(|f| f.upgrade()).collect(),
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerSixteen {
    pub max_health: f32,
    pub current_health: f32,
    pub stamina: f32,
    pub first_spawn: u8,
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub known_stations: Vec<(String, u32)>,
    pub materials: Vec<String>,
    pub tutorials: Vec<String>,
    pub uniques: Vec<String>,
    pub trophies: Vec<String>,
    pub beard_type: BeardStyle,
    pub hair_type: HairStyle,
    pub skin: Color,
    pub hair: Color,
    pub gender: Gender,
    pub food: Vec<Food16>,
}

impl PlayerSixteen {
    #[inline]
    pub fn upgrade(self) -> PlayerSeventeen {
        let PlayerSixteen {
            max_health,
            current_health,
            stamina,
            first_spawn,
            inventory,
            recipes,
            known_stations,
            materials,
            tutorials,
            uniques,
            trophies,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food,
        } = self;
        PlayerSeventeen {
            max_health,
            current_health,
            stamina,
            first_spawn,
            inventory,
            recipes,
            known_stations,
            materials,
            tutorials,
            uniques,
            trophies,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food,
            skills: CharacterSkills::new(),
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerSeventeen {
    pub max_health: f32,
    pub current_health: f32,
    pub stamina: f32,
    pub first_spawn: u8,
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub known_stations: Vec<(String, u32)>,
    pub materials: Vec<String>,
    pub tutorials: Vec<String>,
    pub uniques: Vec<String>,
    pub trophies: Vec<String>,
    pub beard_type: BeardStyle,
    pub hair_type: HairStyle,
    pub skin: Color,
    pub hair: Color,
    pub gender: Gender,
    pub food: Vec<Food16>,
    pub skills: CharacterSkills,
}

impl PlayerSeventeen {
    #[inline]
    pub fn upgrade(self) -> PlayerEighteen {
        let PlayerSeventeen {
            max_health,
            current_health,
            stamina,
            first_spawn,
            inventory,
            recipes,
            known_stations,
            materials,
            tutorials,
            uniques,
            trophies,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food,
            skills,
        } = self;
        PlayerEighteen {
            max_health,
            current_health,
            stamina,
            first_spawn,
            inventory,
            recipes,
            known_stations,
            materials,
            tutorials,
            uniques,
            trophies,
            biomes: Vec::new(),
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food,
            skills,
        }
    }
}
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerEighteen {
    pub max_health: f32,
    pub current_health: f32,
    pub stamina: f32,
    pub first_spawn: u8,
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub known_stations: Vec<(String, u32)>,
    pub materials: Vec<String>,
    pub tutorials: Vec<String>,
    pub uniques: Vec<String>,
    pub trophies: Vec<String>,
    pub biomes: Vec<BiomeId>,
    pub beard_type: BeardStyle,
    pub hair_type: HairStyle,
    pub skin: Color,
    pub hair: Color,
    pub gender: Gender,
    pub food: Vec<Food16>,
    pub skills: CharacterSkills,
}

impl PlayerEighteen {
    #[inline]
    pub fn upgrade(self) -> PlayerNineteen {
        let PlayerEighteen {
            max_health,
            current_health,
            stamina,
            first_spawn,
            inventory,
            recipes,
            known_stations,
            materials,
            tutorials,
            uniques,
            trophies,
            biomes,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food,
            skills,
        } = self;
        let _ = tutorials;
        PlayerNineteen {
            max_health,
            current_health,
            stamina,
            first_spawn,
            inventory,
            recipes,
            known_stations,
            materials,
            uniques,
            trophies,
            biomes,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food,
            skills,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerNineteen {
    pub max_health: f32,
    pub current_health: f32,
    pub stamina: f32,
    pub first_spawn: u8,
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub known_stations: Vec<(String, u32)>,
    pub materials: Vec<String>,
    pub uniques: Vec<String>,
    pub trophies: Vec<String>,
    pub biomes: Vec<BiomeId>,
    pub beard_type: BeardStyle,
    pub hair_type: HairStyle,
    pub skin: Color,
    pub hair: Color,
    pub gender: Gender,
    pub food: Vec<Food16>,
    pub skills: CharacterSkills,
}

impl PlayerNineteen {
    #[inline]
    pub fn upgrade(self) -> PlayerTwenty {
        let PlayerNineteen {
            max_health,
            current_health,
            stamina,
            first_spawn,
            inventory,
            recipes,
            known_stations,
            materials,
            uniques,
            trophies,
            biomes,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food,
            skills,
        } = self;
        PlayerTwenty {
            max_health,
            current_health,
            stamina,
            first_spawn,
            death_timer: 0.0,
            inventory,
            recipes,
            known_stations,
            materials,
            uniques,
            trophies,
            biomes,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food,
            skills,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerTwenty {
    pub max_health: f32,
    pub current_health: f32,
    pub stamina: f32,
    pub first_spawn: u8,
    pub death_timer: f32,
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub known_stations: Vec<(String, u32)>,
    pub materials: Vec<String>,
    pub uniques: Vec<String>,
    pub trophies: Vec<String>,
    pub biomes: Vec<BiomeId>,
    pub beard_type: BeardStyle,
    pub hair_type: HairStyle,
    pub skin: Color,
    pub hair: Color,
    pub gender: Gender,
    pub food: Vec<Food16>,
    pub skills: CharacterSkills,
}

impl PlayerTwenty {
    #[inline]
    pub fn upgrade(self) -> PlayerTwentyOne {
        let PlayerTwenty {
            max_health,
            current_health,
            stamina,
            first_spawn,
            death_timer,
            inventory,
            recipes,
            known_stations,
            materials,
            uniques,
            trophies,
            biomes,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food,
            skills,
        } = self;
        PlayerTwentyOne {
            max_health,
            current_health,
            stamina,
            first_spawn,
            death_timer,
            inventory,
            recipes,
            known_stations,
            materials,
            tutorials: Vec::new(),
            uniques,
            trophies,
            biomes,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food,
            skills,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerTwentyOne {
    pub max_health: f32,
    pub current_health: f32,
    pub stamina: f32,
    pub first_spawn: u8,
    pub death_timer: f32,
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub known_stations: Vec<(String, u32)>,
    pub materials: Vec<String>,
    pub tutorials: Vec<String>,
    pub uniques: Vec<String>,
    pub trophies: Vec<String>,
    pub biomes: Vec<BiomeId>,
    pub beard_type: BeardStyle,
    pub hair_type: HairStyle,
    pub skin: Color,
    pub hair: Color,
    pub gender: Gender,
    pub food: Vec<Food16>,
    pub skills: CharacterSkills,
}

impl PlayerTwentyOne {
    #[inline]
    pub fn upgrade(self) -> PlayerTwentyTwo {
        let PlayerTwentyOne {
            max_health,
            current_health,
            stamina,
            first_spawn,
            death_timer,
            inventory,
            recipes,
            known_stations,
            materials,
            tutorials,
            uniques,
            trophies,
            biomes,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food,
            skills,
        } = self;
        PlayerTwentyTwo {
            max_health,
            current_health,
            stamina,
            first_spawn,
            death_timer,
            inventory,
            recipes,
            known_stations,
            materials,
            tutorials,
            uniques,
            trophies,
            biomes,
            known_text: Vec::new(),
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food,
            skills,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerTwentyTwo {
    pub max_health: f32,
    pub current_health: f32,
    pub stamina: f32,
    pub first_spawn: u8,
    pub death_timer: f32,
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub known_stations: Vec<(String, u32)>,
    pub materials: Vec<String>,
    pub tutorials: Vec<String>,
    pub uniques: Vec<String>,
    pub trophies: Vec<String>,
    pub biomes: Vec<BiomeId>,
    pub known_text: Vec<(String, String)>,
    pub beard_type: BeardStyle,
    pub hair_type: HairStyle,
    pub skin: Color,
    pub hair: Color,
    pub gender: Gender,
    pub food: Vec<Food16>,
    pub skills: CharacterSkills,
}

impl PlayerTwentyTwo {
    #[inline]
    pub fn upgrade(self) -> PlayerTwentyThree {
        let PlayerTwentyTwo {
            max_health,
            current_health,
            stamina,
            first_spawn,
            death_timer,
            inventory,
            recipes,
            known_stations,
            materials,
            tutorials,
            uniques,
            trophies,
            biomes,
            known_text,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food,
            skills,
        } = self;
        PlayerTwentyThree {
            max_health,
            current_health,
            stamina,
            first_spawn,
            death_timer,
            god_power: String::new(),
            inventory,
            recipes,
            known_stations,
            materials,
            tutorials,
            uniques,
            trophies,
            biomes,
            known_text,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food,
            skills,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerTwentyThree {
    pub max_health: f32,
    pub current_health: f32,
    pub stamina: f32,
    pub first_spawn: u8,
    pub death_timer: f32,
    pub god_power: String,
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub known_stations: Vec<(String, u32)>,
    pub materials: Vec<String>,
    pub tutorials: Vec<String>,
    pub uniques: Vec<String>,
    pub trophies: Vec<String>,
    pub biomes: Vec<BiomeId>,
    pub known_text: Vec<(String, String)>,
    pub beard_type: BeardStyle,
    pub hair_type: HairStyle,
    pub skin: Color,
    pub hair: Color,
    pub gender: Gender,
    pub food: Vec<Food16>,
    pub skills: CharacterSkills,
}

impl PlayerTwentyThree {
    #[inline]
    pub fn upgrade(self) -> PlayerTwentyFour {
        let PlayerTwentyThree {
            max_health,
            current_health,
            stamina,
            first_spawn,
            death_timer,
            god_power,
            inventory,
            recipes,
            known_stations,
            materials,
            tutorials,
            uniques,
            trophies,
            biomes,
            known_text,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food,
            skills,
        } = self;
        PlayerTwentyFour {
            max_health,
            current_health,
            stamina,
            first_spawn,
            death_timer,
            god_power,
            power_cooldown: 0.0,
            inventory,
            recipes,
            known_stations,
            materials,
            tutorials,
            uniques,
            trophies,
            biomes,
            known_text,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food,
            skills,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerTwentyFour {
    pub max_health: f32,
    pub current_health: f32,
    pub stamina: f32,
    pub first_spawn: u8,
    pub death_timer: f32,
    pub god_power: String,
    pub power_cooldown: f32,
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub known_stations: Vec<(String, u32)>,
    pub materials: Vec<String>,
    pub tutorials: Vec<String>,
    pub uniques: Vec<String>,
    pub trophies: Vec<String>,
    pub biomes: Vec<BiomeId>,
    pub known_text: Vec<(String, String)>,
    pub beard_type: BeardStyle,
    pub hair_type: HairStyle,
    pub skin: Color,
    pub hair: Color,
    pub gender: Gender,
    pub food: Vec<Food16>,
    pub skills: CharacterSkills,
}

impl PlayerTwentyFour {
    #[inline]
    pub fn upgrade(self) -> PlayerTwentyFive {
        let PlayerTwentyFour {
            max_health,
            current_health,
            stamina,
            first_spawn,
            death_timer,
            god_power,
            power_cooldown,
            inventory,
            recipes,
            known_stations,
            materials,
            tutorials,
            uniques,
            trophies,
            biomes,
            known_text,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food,
            skills,
        } = self;
        PlayerTwentyFive {
            max_health,
            current_health,
            stamina,
            first_spawn,
            death_timer,
            god_power,
            power_cooldown,
            inventory,
            recipes,
            known_stations,
            materials,
            tutorials,
            uniques,
            trophies,
            biomes,
            known_text,
            beard_type,
            hair_type,
            skin,
            hair,
            gender,
            food: food.into_iter().map(|f| f.upgrade()).collect(),
            skills,
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerTwentyFive {
    pub max_health: f32,
    pub current_health: f32,
    pub stamina: f32,
    pub first_spawn: u8,
    pub death_timer: f32,
    pub god_power: String,
    pub power_cooldown: f32,
    pub inventory: CharacterInventory,
    pub recipes: Vec<String>,
    pub known_stations: Vec<(String, u32)>,
    pub materials: Vec<String>,
    pub tutorials: Vec<String>,
    pub uniques: Vec<String>,
    pub trophies: Vec<String>,
    pub biomes: Vec<BiomeId>,
    pub known_text: Vec<(String, String)>,
    pub beard_type: BeardStyle,
    pub hair_type: HairStyle,
    pub skin: Color,
    pub hair: Color,
    pub gender: Gender,
    pub food: Vec<Food25>,
    pub skills: CharacterSkills,
}
