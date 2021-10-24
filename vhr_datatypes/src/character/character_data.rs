use serde::{Deserialize, Serialize};

use crate::prelude::*;

use super::food::*;
use super::version_enum::*;
use super::Compendium;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct CharacterData {
    pub length: u32,
    pub inner: CharacterDataTemp,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct CharacterDataPreHH {
    pub max_hp: f32,
    pub current_hp: f32,
    pub stamina: f32,
    pub first_spawn: u8,
    pub alive_timer: f32,
    pub selected_power: String,
    pub cooldown: f32,
    pub inventory: CharacterInventory,
    pub compendium: Compendium,
    pub beard_type: String,
    pub hair_type: String,
    pub skin: Color,
    pub hair: Color,
    pub gender: Gender,
    pub stomach: Vec<FoodBoth>,
    pub skills: CharacterSkills,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct CharacterDataPostHH {
    pub max_hp: f32,
    pub current_hp: f32,
    pub stamina: f32,
    pub first_spawn: u8,
    pub alive_timer: f32,
    pub selected_power: String,
    pub cooldown: f32,
    pub inventory: CharacterInventory,
    pub compendium: Compendium,
    pub beard_type: String,
    pub hair_type: String,
    pub skin: Color,
    pub hair: Color,
    pub gender: Gender,
    pub stomach: Vec<FoodSingle>,
    pub skills: CharacterSkills,
}
