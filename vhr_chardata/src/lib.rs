use druid::{Data, Lens};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::rc::Rc;
use std::sync::Arc;

pub mod color;
pub mod hair;
pub mod skill;
mod vis_data;

pub use color::Color;
pub use skill::Skill;

pub use vis_data::VisibilityData;

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
    pub skills: Arc<Vec<Skill>>,
}

// fn map_compare(a: &Vec<Map>, b: &Vec<Map>) -> bool {
//     a.eq(b)
// }

impl LoadedCharacter {
    pub fn to_disk(&self) -> CharacterOnDisk {
        CharacterOnDisk {
            data_size: 8192,
            thirty_tree: 33,
            unknown: 0,
            death_count: self.death_count,
            crafting_count: self.crafting_count,
            building_count: self.building_count,
            maps: (*self.maps).clone(),
            name: self.name.clone(),
            player_id: self.player_id,
            unknown_a: 0,
            unknown_b: 0,
            character_data_length: 8192,
            always_twenty_four: 24,
            max_hp: self.max_hp,
            current_hp: self.current_hp,
            stamina: self.stamina,
            play_into: self.play_into,
            alive_timer: self.alive_timer,
            selected_power: self.selected_power.clone(),
            cooldown: self.cooldown,
            always_one_o_three: 103,
            inventory: vec![],
            compendium: (*self.compendium).clone(),
            beard_type: self.beard_type.clone(),
            hair_type: self.hair_type.clone(),
            skin: self.skin.clone(),
            hair: self.hair.clone(),
            gender: self.gender.clone(),
            stomach: (*self.stomach).clone(),
            always_two: 2,
            skills: (*self.skills).clone(),
            hash: vec![],
        }
    }
}

impl Default for LoadedCharacter {
    fn default() -> Self {
        LoadedCharacter {
            kill_count: 2,
            death_count: 94,
            crafting_count: 502,
            building_count: 87125,
            maps: Rc::new(vec![Map::default()]),
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
            skills: Arc::new(vec![Skill::AXES, Skill::RUN, Skill::SNEAK]),
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

impl CharacterOnDisk {
    pub fn pre_serialize(&mut self) -> usize {
        let mut size: usize = 37;
        size += self.selected_power.len() + 1;
        size += self
            .inventory
            .iter_mut()
            .map(|i| i.pre_serialize())
            .sum::<usize>();
        size += self.compendium.pre_serialize();
        size += self.beard_type.len() + 1;
        size += self.hair_type.len() + 1;
        size += self
            .stomach
            .iter_mut()
            .map(|f| f.pre_serialize())
            .sum::<usize>();
        size += self
            .skills
            .iter_mut()
            .map(|s| s.pre_serialize())
            .sum::<usize>();
        self.character_data_length = size as u32;
        // everything above this is part of the character_data_length
        size += 30;
        size += self
            .maps
            .iter_mut()
            .map(|m| m.pre_serialize())
            .sum::<usize>();
        size += self.name.len() + 1;
        self.data_size = size as u32;
        // everything above here is part of the hashed data packet
        // todo: actually hash this
        self.hash = vec![0xDE; 64];
        size += self.hash.len() + 4 + 4;
        size
    }
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

impl Item {
    fn pre_serialize(&mut self) -> usize {
        let mut size = 33; //hand-counted size of struct
        size += self.name.len() + 1;
        size += self.creator_name.len() + 1;
        size
    }
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

impl Compendium {
    fn pre_serialize(&mut self) -> usize {
        self.recipes.len()
            + 4
            + self
                .craftbenches
                .iter()
                .map(|c| c.0.len() + 1 + 4)
                .sum::<usize>()
            + self
                .materials_list
                .iter()
                .map(|m| m.len() + 1)
                .sum::<usize>()
            + self.places.iter().map(|m| m.len() + 1).sum::<usize>()
            + self.unknown_list.iter().map(|m| m.len() + 1).sum::<usize>()
            + self.trophies.iter().map(|m| m.len() + 1).sum::<usize>()
            + (self.biomes.len() * 4)
            + self
                .tutorials
                .iter()
                .map(|(a, b)| a.len() + b.len() + 2)
                .sum::<usize>()
    }
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

impl Map {
    pub fn pre_serialize(&mut self) -> usize {
        let mut size: usize = self.fog_of_war.pre_serialize();
        size += self
            .pois
            .iter_mut()
            .map(|p| p.pre_serialize())
            .sum::<usize>()
            + 4;
        if let Some(_) = self.extra {
            size += 1;
        }
        self.map_size = size as u32;
        size + 24
    }
}

#[derive(Default, Data, Clone, Lens, PartialEq, Debug, Serialize, Deserialize)]
pub struct Food {
    name: String,
    health: f32,
    stamina: f32,
}

impl Food {
    fn pre_serialize(&mut self) -> usize {
        self.name.len() + 1 + 8
    }
}

#[derive(Default, Data, Clone, Lens, PartialEq, Debug, Serialize, Deserialize)]
pub struct Poi {
    name: String,
    pos: Point,
    kind: u8,
    flags: u32,
}
impl Poi {
    pub fn pre_serialize(&mut self) -> usize {
        let size = self.name.len() + 1;
        size + 8
    }
}

#[derive(Default, Data, Clone, Lens, PartialEq, Debug, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
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
