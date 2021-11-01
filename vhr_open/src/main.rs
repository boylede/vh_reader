use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{Read, Seek, Write};

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

    println!("{:#?}", character);
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
