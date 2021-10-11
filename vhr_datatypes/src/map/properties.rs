use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

const PROPERTY_NAMES: &[&str] = &include!("../../../assets/data/property_names.txt");

lazy_static! {
    static ref PROPERTYNAME_MAP: HashMap<u32, &'static str> = {
        PROPERTY_NAMES.iter().map(|v|(hash_str(v), *v)).collect()
    };
}

pub fn get_property_name(hash: u32) -> Option<&'static str> {
    PROPERTYNAME_MAP.get(&hash).map(|s|*s)
}

#[derive(Hash, Default, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub struct PropertyName{
    #[serde(skip)]
    order: usize,
    id: u32,
}

impl std::fmt::Debug for PropertyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let name = get_property_name(self.id);
        if let Some(name) = name {
            write!(f, "{}!:", name)
        } else {
            write!(f, "Unknown Property")
        }
    }
}

const SEED: u32 = 5381;
const MULTIPLIER: u32 = 1566083941;


/// the iterator version of the hashing function used for property names
fn hash_str(s: &str) -> u32 {
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

/// The const version of the hashing function used for property names
/// (const fns cannot use iterators currently)
const fn const_hash_str(s: &str) -> u32 {
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
    assert!(PROPERTY_NAMES
        .iter()
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
