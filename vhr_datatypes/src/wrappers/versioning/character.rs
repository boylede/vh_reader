use crate::prelude::*;
use serde::{Deserialize, Serialize};

use crate::wrappers::versioning::{NoSuchVersion, UnknownVersion};

/// an enum with all versions of the player's profile data type
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum CharacterFileVersions {
    Zero(NoSuchVersion),
    One(UnknownVersion),
    Two(UnknownVersion),
    Three(UnknownVersion),
    Four(UnknownVersion),
    Five(UnknownVersion),
    Six(UnknownVersion),
    Seven(UnknownVersion),
    Eight(UnknownVersion),
    Nine(UnknownVersion),
    Ten(UnknownVersion),
    Eleven(UnknownVersion),
    Twelve(UnknownVersion),
    Thirteen(UnknownVersion),
    Fourteen(UnknownVersion),
    Fifteen(UnknownVersion),
    Sixteen(UnknownVersion),
    Seventeen(UnknownVersion),
    Eighteen(UnknownVersion),
    NineTeen(UnknownVersion),
    Twenty(UnknownVersion),
    TwentyOne(UnknownVersion),
    TwentyTwo(UnknownVersion),
    TwentyThree(UnknownVersion),
    TwentyFour(UnknownVersion),
    TwentyFive(UnknownVersion),
    TwentySix(UnknownVersion),
    TwentySeven(Character),
    TwentyEight(Character),
    TwentyNine(Character),
    Thirty(Character),
    ThirtyOne(Character),
    ThirtyTwo(Character),
    ThirtyThree(Character),
    ThirtyFour(Character),
    ThirtyFive(Character),
    ThirtySix(Character),
}

impl CharacterFileVersions {
    pub fn to_inner(self) -> Option<Character> {
        use CharacterFileVersions::*;
        match self {
            TwentySeven(profile) => Some(profile),
            TwentyEight(profile) => Some(profile),
            TwentyNine(profile) => Some(profile),
            Thirty(profile) => Some(profile),
            ThirtyOne(profile) => Some(profile),
            ThirtyTwo(profile) => Some(profile),
            ThirtyThree(profile) => Some(profile),
            ThirtyFour(profile) => Some(profile),
            ThirtyFive(profile) => Some(profile),
            ThirtySix(profile) => Some(profile),
            _ => None,
        }
    }
    pub fn wrap_latest(inner: Character) -> Self {
        CharacterFileVersions::ThirtySix(inner)
    }
}
