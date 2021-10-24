use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{Read, Seek, Write};
use vhr_datatypes::map::{hashed_string::HashedString, Entity, MapDatabaseFile};

fn main() {
    // let collisions = vhr_datatypes::map::properties::find_collisions();
    // for c in collisions.iter() {
    //     println!("{:#?}", c);
    // }

    let Config { filename } =
        read_commandline().expect("Couldn't understand command line arguments");

    let mut loaded_file = Vec::new();
    {
        let mut in_file = File::open(&filename).expect("Failed opening file");
        in_file.read_to_end(&mut loaded_file).ok();
    }

    // for i in 0..16 {
    //     print!("{:x} ", loaded_file[i]);
    // }
    // println!("");

    let mut map: MapDatabaseFile =
        vhr_serde::de::from_bytes(&loaded_file).expect("Couldn't deserialize map.");

    // println!("{:#?}", map);
    // let prefabs = count_prefabs(&map);
    // print_prefab_names(&prefabs);

    print_property_names(&map);
    // vhr_datatypes::map::hashed_string::print_unique_strings();
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

fn print_property_names(map: &MapDatabaseFile) {
    println!("[");
    let names: HashSet<HashedString> = map
        .entities
        .entities
        .iter()
        .flat_map(|e| {
            let f_names = e.floats.iter().map(|p| *p.0);
            let p_names = e.points.iter().map(|p| *p.0);
            let r_names = e.rots.iter().map(|p| *p.0);
            let i_names = e.ints.iter().map(|p| *p.0);
            let pair_names = e.pairs.iter().map(|p| *p.0);
            let s_names = e.strings.iter().map(|p| *p.0);
            let b_names = e.bytes.iter().map(|p| *p.0);
            f_names
                .chain(p_names)
                .chain(r_names)
                .chain(i_names)
                .chain(pair_names)
                .chain(s_names)
                .chain(b_names)
        })
        .collect();
    for name in names.iter() {
        println!("{:?}", name);
    }
    println!("]");
}

fn print_prefab_names(prefabs: &HashMap<HashedString, u32>) {
    let prefab_names: Vec<String> = prefabs.iter().map(|(k, v)| k.to_string()).collect();
    println!("[");
    for name in prefab_names.iter() {
        println!("{},", name);
    }
    println!("]");
}

fn count_prefabs(map: &MapDatabaseFile) -> HashMap<HashedString, u32> {
    let prefabs: HashMap<HashedString, u32> =
        map.entities.entities.iter().map(|e| e.prefab_id).fold(
            HashMap::new(),
            |mut previous, next| {
                *previous.entry(next).or_insert(0) += 1;
                previous
            },
        );
    prefabs
}
fn print_entities_in_one_sector(map: &MapDatabaseFile) {
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
}

fn save_test_output_map(map: &MapDatabaseFile) {
    let out_filename = "test_out.db";
    let mut out_file = File::create(&out_filename).expect("Failed opening file");
    let output_bytes = vhr_serde::ser::to_bytes(&map).expect("Failed to serialize map");
    out_file
        .write_all(&output_bytes)
        .expect("Failed to write output file");
}
