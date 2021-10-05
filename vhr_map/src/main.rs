use std::env;
use std::fs::File;
use std::io::{Read, Seek, Write};

use vhr_datatypes::map::{Entity, MapDatabaseFile};

fn main() {
    let Config { filename } =
        read_commandline().expect("Couldn't understand command line arguments");

    let mut loaded_file = Vec::new();
    {
        let mut in_file = File::open(&filename).expect("Failed opening file");
        in_file.read_to_end(&mut loaded_file).ok();
    }

    for i in 0..16 {
        print!("{:x} ", loaded_file[i]);
    }
    println!("");

    let mut map: MapDatabaseFile =
        vhr_serde::de::from_bytes(&loaded_file).expect("Couldn't deserialize map.");

    // println!("{:?}", map);
    // let mut world = map.load();
    let smallest_sector: Option<(i32, i32)> = map
        .entities
        .entities
        .iter()
        .map(|e| (e.sector_x, e.sector_y))
        .min_by_key(|&(x, y)| ((x as f32).powi(2) * (y as f32).powi(2)).sqrt() as i32);

    if let Some((x, y)) = smallest_sector {
        let things: Vec<Entity> = map
            .entities
            .entities
            .iter()
            .filter(|e| e.sector_x == x && e.sector_y == y)
            .cloned()
            .collect();
        println!("{},{} sector: {:#?}", x, y, things);
    }

    // let new_map = world.store();
    {
        let out_filename = "test_out.db";
        let mut out_file = File::create(&out_filename).expect("Failed opening file");
        let output_bytes = vhr_serde::ser::to_bytes(&map).expect("Failed to serialize map");
        out_file
            .write_all(&output_bytes)
            .expect("Failed to write output file");
    }
}

struct Config {
    filename: String,
}

#[derive(Debug)]
enum CommandLineError {
    NotEnoughArgs,
    CouldntParseArgs,
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
