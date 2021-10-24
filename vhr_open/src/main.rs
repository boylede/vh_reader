use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{Read, Seek, Write};

use vhr_datatypes::character::*;
use vhr_datatypes::map::{hashed_string::HashedString, Entity, MapDatabaseFile};
use vhr_datatypes::prelude::*;

fn main() {
    let Config { filename } =
        read_commandline().expect("Couldn't understand command line arguments");

    let mut loaded_file = Vec::new();
    {
        let mut in_file = File::open(&filename).expect("Failed opening file");
        in_file.read_to_end(&mut loaded_file).ok();
    }

    let mut character: CharacterFile =
        vhr_serde::de::from_bytes(&loaded_file).expect("Couldn't deserialize file");

    if let Some(profile) = match character.inner {
        PlayerProfile::ThirtyThree(profile) => Some(profile),
        PlayerProfile::ThirtySix(profile) => Some(profile),
        _ => None,
    } {
        let kill_count = profile.kill_count;
        let death_count = profile.death_count;
        let crafting_count = profile.crafting_count;
        let building_count = profile.building_count;
        let name = profile.name;
        let player_id = profile.player_id;
        print!("Name: {}\nid: {}\nKill count: {}\nDeath count: {}\nCrafting count: {}\nBuilding count: {}\n",
    name, player_id, kill_count, death_count, crafting_count, building_count);
        let maps: Vec<Map> = profile.maps;
        for (i, map) in maps.iter().enumerate() {
            println!("Map # {}", i);
            println!("\tid: {}", map.id);
            println!("\tspawn point: {:?}", map.spawn);
            println!("\tcurrent pos: {:?}", map.position);
            println!("\tdeath pos: {:?}", map.death);
            println!("\thome: {:?}", map.home);
            if let Some(mini) = &map.mini_map {
                println!("\tmini map size: {}", mini.len());
            }
        }
        if let Some(data) = profile.data {
            use vhr_datatypes::character::CharacterDataTemp::*;
            match data.inner {
                TwentyFour(data) => {
                    println!("found version 24 character data");
                    use vhr_datatypes::character::character_data::CharacterDataPreHH;
                    // let CharacterDataPreHH {
                    //     max_hp,
                    //     current_hp,
                    //     stamina,
                    //     first_spawn,
                    //     alive_timer,
                    //     selected_power,
                    //     cooldown,
                    //     inventory,
                    //     compendium,
                    //     beard_type,
                    //     hair_type,
                    //     skin,
                    //     hair,
                    //     gender,
                    //     stomach,
                    //     skills,
                    // } = data;
                    println!("{:#?}", data);
                }
                TwentyFive(data) => {
                    println!("found version 25 character data");
                    use vhr_datatypes::character::character_data::CharacterDataPostHH;
                    // let CharacterDataPostHH {
                    //     max_hp,
                    //     current_hp,
                    //     stamina,
                    //     first_spawn,
                    //     alive_timer,
                    //     selected_power,
                    //     cooldown,
                    //     inventory,
                    //     compendium,
                    //     beard_type,
                    //     hair_type,
                    //     skin,
                    //     hair,
                    //     gender,
                    //     stomach,
                    //     skills,
                    // } = data;
                    println!("{:#?}", data);
                }
                _ => (),
            }
        }
    };
}

struct Config {
    filename: String,
}

#[derive(Debug)]
enum CommandLineError {
    NotEnoughArgs,
}

fn read_commandline() -> Result<Config, CommandLineError> {
    let mut args = env::args();
    args.next(); // skip command filename
    let filename: String = match args.next() {
        Some(s) => s,
        None => return Err(CommandLineError::NotEnoughArgs),
    };

    Ok(Config { filename })
}
