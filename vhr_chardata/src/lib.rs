use druid::{Data, Lens};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::rc::Rc;

#[derive(Data, Clone, Lens, Debug)]
pub struct LoadedCharacter {
    pub kill_count: u32,
    pub death_count: u32,
    pub crafting_count: u32,
    pub building_count: u32,
    pub maps: Rc<Vec<Map>>,
    pub name: String,
    pub player_id: u64,
    pub max_hp: f32,
    pub current_hp: f32,
    pub stamina: f32,
    pub play_into: u8,
    pub alive_timer: f32,
    pub selected_power: String,
    pub cooldown: f32,
    pub inventory: Rc<Vec<Item>>,
    pub compendium: Rc<Compendium>,
    pub beard_type: String,
    pub hair_type: String,
    pub skin: Color,
    pub hair: Color,
    pub gender: Gender,
    pub stomach: Rc<Vec<Food>>,
    pub skills: Rc<Vec<Skill>>,
}

impl Default for LoadedCharacter {
    fn default() -> Self {
        LoadedCharacter {
            kill_count: 2,
            death_count: 94,
            crafting_count: 502,
            building_count: 87125,
            maps: Rc::new(vec![]),
            name: "Bobbleth".into(),
            player_id: 5,
            max_hp: 52.2,
            current_hp: 50.0,
            stamina: 100.0,
            play_into: 0,
            alive_timer: 1000.0,
            selected_power: "GP_Eikthyr".into(),
            cooldown: 0.0,
            inventory: Rc::new(vec![]),
            compendium: Rc::new(Compendium {
                recipes: vec![],
                craftbenches: vec![],
                materials_list: vec![],
                places: vec![],
                unknown_list: vec![],
                trophies: vec![],
                biomes: vec![],
                tutorials: vec![],
            }),
            beard_type: "Beard01".into(),
            hair_type: "Hair02".into(),
            skin: Color::BROWN,
            hair: Color::WHITE,
            gender: Gender::Female,
            stomach: Rc::new(vec![]),
            skills: Rc::new(vec![]),
        }
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct CharacterOnDisk {
    pub data_size: u32,
    pub thirty_tree: u32,
    pub unknown: u32,
    pub death_count: u32,
    pub crafting_count: u32,
    pub building_count: u32,
    pub maps: Vec<Map>,
    pub name: String,
    pub player_id: u64,
    pub unknown_a: u8,
    pub unknown_b: u8,
    pub character_data_length: u32,
    pub always_twenty_four: u32,
    pub max_hp: f32,
    pub current_hp: f32,
    pub stamina: f32,
    pub play_into: u8,
    pub alive_timer: f32,
    pub selected_power: String,
    pub cooldown: f32,
    pub always_one_o_three: u32,
    pub inventory: Vec<Item>,
    pub compendium: Compendium,
    pub beard_type: String,
    pub hair_type: String,
    pub skin: Color,
    pub hair: Color,
    pub gender: Gender,
    pub stomach: Vec<Food>,
    pub always_two: u32,
    pub skills: Vec<Skill>,
    pub hash: Vec<u8>,
}

#[derive(Default, Data, Clone, Lens, PartialEq, Debug, Serialize, Deserialize)]
pub struct Item {
    name: String,
    quantity: u32,
    durability: f32,
    column: u32,
    row: u32,
    equipped: u8,
    quality: u32,
    variant: u32,
    creator_id: u64,
    creator_name: String,
}

#[derive(Default, Clone, Lens, PartialEq, Debug, Serialize, Deserialize)]
pub struct Compendium {
    pub recipes: Vec<String>,
    pub craftbenches: Vec<(String, u32)>,
    pub materials_list: Vec<String>,
    pub places: Vec<String>,
    pub unknown_list: Vec<String>,
    pub trophies: Vec<String>,
    pub biomes: Vec<Biomes>,
    pub tutorials: Vec<(String, String)>,
}

#[derive(Default, Clone, Lens, PartialEq, Debug, Serialize, Deserialize)]
pub struct Map {
    unknown: [u8; 9],
    point_a: Point,
    always_one_a: u8,
    current_position: Point,
    always_zero: u8,
    point_b: Point,
    point_c: Point,
    always_one_b: u8,
    map_size: u32,
    fog_of_war: VisibilityData,
    pois: Vec<Poi>,
    // possible extra byte for some reason, but only sometimes:
    extra: Option<u8>,
}

#[derive(Default, Data, Clone, Lens, PartialEq, Debug, Serialize, Deserialize)]
pub struct Food {
    name: String,
    health: f32,
    stamina: f32,
}

#[derive(Default, Data, Clone, Lens, PartialEq, Debug, Serialize, Deserialize)]
pub struct Poi {
    name: String,
    pos: Point,
    kind: u8,
    flags: u32,
}

#[derive(Default, Data, Clone, Lens, PartialEq, Debug, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Default, Data, Clone, Lens, PartialEq, Debug, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };
    pub const BROWN: Color = Color {
        r: 0.4,
        g: 0.2,
        b: 0.2,
    };
}

#[derive(PartialEq, Eq, Data, Clone, Lens, Debug, Deserialize, Serialize)]
pub struct VisibilityData {
    four: u32,
    twenty_fourty_eight: u32,
    // todo: remove box by specifying length; requires custom serde impls because len > 32
    fog: Rc<Box<[u8]>>,
}

impl Default for VisibilityData {
    fn default() -> Self {
        VisibilityData {
            four: 4,
            twenty_fourty_eight: 2048,
            fog: Rc::new(Box::new([0u8; FOG_LENGTH])),
        }
    }
}

const FOG_LENGTH: usize = 0x400000;

#[derive(PartialEq, Eq, Data, Clone, Debug, Serialize, Deserialize)]
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

#[derive(Data, Clone, Lens, PartialEq, Debug, Serialize, Deserialize)]
pub struct Skill {
    id: SkillName,
    level: f32,
    progress: f32,
}

#[derive(PartialEq, Eq, Data, Clone, Debug, Serialize, Deserialize)]
#[repr(u32)]
#[non_exhaustive]
pub enum Biomes {
    None = 0,
    Meadows = 1,
    Blackforest = 8,
}

impl Default for Biomes {
    fn default() -> Self {
        Biomes::None
    }
}

#[derive(PartialEq, Eq, Data, Clone, Debug, Serialize_repr, Deserialize_repr)]
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

#[derive(PartialEq, Eq, Data, Clone)]
#[non_exhaustive]
pub enum BeardType {
    Beard1,
    Beard2,
}

#[derive(PartialEq, Eq, Data, Clone)]
#[non_exhaustive]
pub enum HairType {
    Hair1,
    Hair2,
}
