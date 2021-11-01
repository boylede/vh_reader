use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(PartialEq, Eq, Clone, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
#[non_exhaustive]
pub enum SkillName {
    None = 0,
    Swords = 1,
    Knives = 2,
    Clubs = 3,
    Polearms = 4,
    Spears = 5,
    Blocking = 6,
    Axes = 7,
    Bows = 8,
    // 9
    // 10
    // 11
    Unarmed = 11,
    Pickaxes = 12,
    Woodcutting = 13,
    // SKIP A FEW..?
    Jump = 100,
    Sneak = 101,
    Run = 102,
    Swim = 103,
}

impl Default for SkillName {
    fn default() -> Self {
        SkillName::None
    }
}

impl std::fmt::Display for SkillName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct ParseErr;
impl std::fmt::Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError")?;
        Ok(())
    }
}

impl std::error::Error for ParseErr {}

impl std::str::FromStr for SkillName {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, ParseErr> {
        use SkillName::*;
        let name = match s {
            "None" => None,
            "Swords" => Swords,
            "Knives" => Knives,
            "Clubs" => Clubs,
            "Polearms" => Polearms,
            "Spears" => Spears,
            "Blocking" => Blocking,
            "Axes" => Axes,
            "Bows" => Bows,
            // 9
            // 10
            "Unarmed" => Unarmed,
            "Pickaxes" => Pickaxes,
            "Woodcutting" => Woodcutting,
            // Skip A Few..?
            "Jump" => Jump,
            "Sneak" => Sneak,
            "Run" => Run,
            "Swim" => Swim,
            _ => None,
        };
        Ok(name)
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Skill {
    pub id: SkillName,
    pub level: f32,
    pub progress: f32,
}

impl Skill {
    pub const AXES: Skill = Skill {
        id: SkillName::Axes,
        level: 2.0,
        progress: 50.0,
    };
    pub const RUN: Skill = Skill {
        id: SkillName::Run,
        level: 5.0,
        progress: 67.0,
    };
    pub const SNEAK: Skill = Skill {
        id: SkillName::Sneak,
        level: 100.0,
        progress: 25.0,
    };
    pub const NONE: Skill = Skill {
        id: SkillName::None,
        level: 0.0,
        progress: 0.0,
    };
}
