use serde::{Deserialize, Serialize};

/// a list of currently known food names in the game and their respective duration, in seconds.
pub const KNOWN_FOODS: &[(&'static str, f32)] = &[
    ("Raspberry", 600.0),
    ("Mushroom", 900.0),
    ("Honey", 900.0),
    ("NeckTailGrilled", 1200.0),
    ("CookedMeat", 1200.0),
    ("BoarJerky", 1800.0),
    ("CookedDeerMeat", 1200.0),
    ("Pukeberries", 15.0),
    ("Blueberries", 600.0),
    ("Carrot", 900.0),
    ("MushroomYellow", 600.0),
    ("CarrotSoup", 1500.0),
    ("QueensJam", 1200.0),
    ("FishCooked", 1200.0),
    ("DeerStew", 1500.0),
    ("MinceMeatSauce", 1500.0),
    ("ShocklateSmoothie", 1200.0),
    ("TurnipStew", 1500.0),
    ("Sausages", 1500.0),
    ("BlackSoup", 1200.0),
    ("SerpentMeatCooked", 2000.0),
    ("SerpentStew", 1800.0),
    ("Onion", 900.0),
    ("OnionSoup", 1200.0),
    ("CookedWolfMeat", 1200.0),
    ("Wolfjerky", 1800.0),
    ("Wolfmeatskewer", 1500.0),
    ("Eyescream", 1500.0),
    ("Cloudberry", 900.0),
    ("CookedLoxMeat", 2000.0),
    ("FishWraps", 1500.0),
    ("LoxPie", 1800.0),
    ("BloodPudding", 1800.0),
    ("Bread", 1500.0),
];

/// food data version 12
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Food12 {
    pub name: String,
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
}

impl Food12 {
    pub fn upgrade(self) -> Food13 {
        let Food12 {
            name,
            a,
            b,
            c,
            d,
            e,
            f,
        } = self;
        Food13 {
            name,
            a,
            b,
            c,
            d,
            e,
            f,
            g: 1.0,
        }
    }
}
/// food data version 13
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Food13 {
    pub name: String,
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
    pub g: f32,
}

impl Food13 {
    pub fn upgrade(self) -> Food14 {
        let Food13 { name, .. } = self;
        Food14 { name, health: 50.0 } // todo: lookup table for values
    }
}
/// food data version 14
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Food14 {
    pub name: String,
    pub health: f32,
}

impl Food14 {
    pub fn upgrade(self) -> Food16 {
        let Food14 { name, health } = self;
        Food16 {
            name,
            health,
            stamina: 50.0,
        } // todo: lookup table for values
    }
}
/// food data version 15 is the same as vesion 15
pub type Food15 = Food14;

/// food data version 16
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Food16 {
    pub name: String,
    pub health: f32,
    pub stamina: f32,
}

impl Food16 {
    pub fn upgrade(self) -> Food25 {
        let Food16 {
            name,
            health,
            stamina,
        } = self;
        let _ = health;
        let _ = stamina;
        let time = if let Some(t) = KNOWN_FOODS
            .iter()
            .filter(|(n, _)| *n == name.as_str())
            .map(|(_, t)| t)
            .next()
        {
            *t
        } else {
            2000.0
        };
        Food25 { name, time }
    }
}

/// food data version 25
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Food25 {
    pub name: String,
    pub time: f32,
}
