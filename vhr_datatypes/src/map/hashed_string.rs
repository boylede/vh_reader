use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

const KNOWN_STRINGS: &str = &include_str!("../../../assets/data/known_strings.txt");

lazy_static! {
    pub(crate) static ref KNOWN_STRING_LOOKUP: HashMap<u32, &'static str> = {
        KNOWN_STRINGS.lines().map(|v|(hash_str(v), v)).collect()
    };
}

pub fn lookup_string(hash: u32) -> Option<&'static str> {
    KNOWN_STRING_LOOKUP.get(&hash).map(|s|*s)
}

pub fn print_unique_strings() {
    let k: HashSet<&str> = KNOWN_STRING_LOOKUP.iter().map(|(k,v)| *v).collect();
    let mut v: Vec<&str> = k.into_iter().collect();
    v.sort();
    for s in v.iter() {
        println!("{}", s);
    }
}

#[derive(Hash, Default, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub struct HashedString{
    #[serde(skip)]
    order: usize, //todo: we want to use this to serialize in the same order as it was deserialized
    id: u32,
}

impl std::fmt::Debug for HashedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let name = lookup_string(self.id);
        if let Some(name) = name {
            write!(f, "\"{}\"", name)
        } else {
            write!(f, "Unknown string #{:#x}", self.id)
        }
    }
}

impl std::fmt::Display for HashedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

const SEED: u32 = 5381;
const MULTIPLIER: u32 = 1566083941;

/// the iterator version of the hashing function used for string IDs
pub fn hash_str(s: &str) -> u32 {
    let (a, b) = s.as_bytes().iter().map(|a| *a as u32).fold(
        (SEED, SEED),
        |(accumulator_a, accumulator_b), next| {
            let new_a = ((accumulator_a << 5).wrapping_add(accumulator_a)) ^ next;
            // swap order so we do b next loop
            (accumulator_b, new_a)
        },
    );
    if s.len() & 0b1 == 0b1 {
        b.wrapping_add(a.wrapping_mul(MULTIPLIER))
    } else {
        a.wrapping_add(b.wrapping_mul(MULTIPLIER))
    }
}

/// The const version of the hashing function used for string IDs
/// (const fns cannot use iterators currently)
pub const fn const_hash_str(s: &str) -> u32 {
    let bytes = s.as_bytes();
    let mut accumulator = [SEED, SEED];
    let mut index = 0;
    let mut which = true;
    loop {
        if index >= bytes.len() {
            break;
        }
        let current = if which { 0 } else { 1 };
        accumulator[current] =
            ((accumulator[current] << 5).wrapping_add(accumulator[current])) ^ bytes[index] as u32;
        which = !which;
        index += 1;
    }
    return accumulator[0].wrapping_add(accumulator[1].wrapping_mul(MULTIPLIER));
}

#[test]
fn check_const_hash_fn() {
    assert!(KNOWN_STRINGS
        .lines()
        .all(|s| { hash_str(s) == const_hash_str(s) }));
}

#[test]
fn check_hash_a() {
    let s = "a";
    let hash = hash_str(s);
    let expected = 372029373;
    println!("{} = {}", s, hash);
    assert!(hash == expected);
}

#[test]
fn check_hash_ab() {
    let s = "ab";
    let hash = hash_str(s);
    let expected = 1093630535;
    println!("{} = {}", s, hash);
    assert!(hash == expected);
}

#[test]
fn check_hash_names() {
    let s = "name5";
    let hash = hash_str(s);
    let expected = 2515777080;
    println!("{} = {}", s, hash);
    assert!(hash == expected);
}
