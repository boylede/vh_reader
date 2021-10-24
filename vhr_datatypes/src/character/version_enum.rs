// in my defense... there is no defense... todo: figure out a way to remove all this boilerplate

use serde::{Deserialize, Serialize};

use super::{character_data::*, mini_map::*, profile::*, Map};
use crate::prelude::*;

/// an unknown data version
/// provides a stub so deserialization can fail
/// todo: write (de)serializer to always fail.
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct UnknownVersion;

/// an enum over all character inventory versions
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum CharacterInventory {
    Zero(UnknownVersion),

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
    TwentySeven(UnknownVersion),
    TwentyEight(UnknownVersion),
    TwentyNine(UnknownVersion),

    Thirty(UnknownVersion),
    ThirtyOne(UnknownVersion),
    ThirtyTwo(UnknownVersion),
    ThirtyThree(UnknownVersion),
    ThirtyFour(UnknownVersion),
    ThirtyFive(UnknownVersion),
    ThirtySix(UnknownVersion),
    ThirtySeven(UnknownVersion),
    ThirtyEight(UnknownVersion),
    ThirtyNine(UnknownVersion),

    Forty(UnknownVersion),
    FortyOne(UnknownVersion),
    FortyTwo(UnknownVersion),
    FortyThree(UnknownVersion),
    FortyFour(UnknownVersion),
    FortyFive(UnknownVersion),
    FortySix(UnknownVersion),
    FortySeven(UnknownVersion),
    FortyEight(UnknownVersion),
    FortyNine(UnknownVersion),

    Fifty(UnknownVersion),
    FiftyOne(UnknownVersion),
    FiftyTwo(UnknownVersion),
    FiftyThree(UnknownVersion),
    FiftyFour(UnknownVersion),
    FiftyFive(UnknownVersion),
    FiftySix(UnknownVersion),
    FiftySeven(UnknownVersion),
    FiftyEight(UnknownVersion),
    FiftyNine(UnknownVersion),

    Sixty(UnknownVersion),
    SixtyOne(UnknownVersion),
    SixtyTwo(UnknownVersion),
    SixtyThree(UnknownVersion),
    SixtyFour(UnknownVersion),
    SixtyFive(UnknownVersion),
    SixtySix(UnknownVersion),
    SixtySeven(UnknownVersion),
    SixtyEight(UnknownVersion),
    SixtyNine(UnknownVersion),

    Seventy(UnknownVersion),
    SeventyOne(UnknownVersion),
    SeventyTwo(UnknownVersion),
    SeventyThree(UnknownVersion),
    SeventyFour(UnknownVersion),
    SeventyFive(UnknownVersion),
    SeventySix(UnknownVersion),
    SeventySeven(UnknownVersion),
    SeventyEight(UnknownVersion),
    SeventyNine(UnknownVersion),

    Eighty(UnknownVersion),
    EightyOne(UnknownVersion),
    EightyTwo(UnknownVersion),
    EightyThree(UnknownVersion),
    EightyFour(UnknownVersion),
    EightyFive(UnknownVersion),
    EightySix(UnknownVersion),
    EightySeven(UnknownVersion),
    EightyEight(UnknownVersion),
    EightyNine(UnknownVersion),

    Ninety(UnknownVersion),
    NinetyOne(UnknownVersion),
    NinetyTwo(UnknownVersion),
    NinetyThree(UnknownVersion),
    NinetyFour(UnknownVersion),
    NinetyFive(UnknownVersion),
    NinetySix(UnknownVersion),
    NinetySeven(UnknownVersion),
    NinetyEight(UnknownVersion),
    NinetyNine(UnknownVersion),

    OneHundred(UnknownVersion),
    OneOhOne(UnknownVersion),
    OneOhTwo(UnknownVersion),
    OneOhThree(Vec<Item>),
}

/// an enum with all versions of the player's profile data type
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum PlayerProfile {
    Zero(UnknownVersion),
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
    TwentySeven(UnknownVersion),
    TwentyEight(UnknownVersion),
    TwentyNine(UnknownVersion),
    Thirty(UnknownVersion),
    ThirtyOne(UnknownVersion),
    ThirtyTwo(UnknownVersion),
    ThirtyThree(Profile),
    ThirtyFour(UnknownVersion),
    ThirtyFive(UnknownVersion),
    ThirtySix(Profile),
    ThirtySeven(UnknownVersion),
    ThirtyEight(UnknownVersion),
    ThirtyNine(UnknownVersion),
}

/// an enum with player skills versions
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum CharacterSkills {
    Zero(UnknownVersion),
    One(UnknownVersion),
    Two(Vec<Skill>),
}

/// an enum with all versions of the player's inner profile data type
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum CharacterDataTemp {
    Zero(UnknownVersion),
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
    TwentyFour(CharacterDataPreHH),
    TwentyFive(CharacterDataPostHH),
}

// /// an enum with all versions of the player's inner profile data type
// #[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
// pub enum VersionedMiniMap {
//     Zero(UnknownVersion),
//     One(UnknownVersion),
//     Two(UnknownVersion),
//     Three(UnknownVersion),
//     Four(UncompressedMiniMap),
//     Five(UnknownVersion),
//     Six(UnknownVersion),
//     Seven(CompressedMiniMap),
// }
