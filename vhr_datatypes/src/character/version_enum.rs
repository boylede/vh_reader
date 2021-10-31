// in my defense... there is no defense... todo: figure out a way to remove all this boilerplate

use serde::{Deserialize, Serialize};

use vhr_serde::de::VHDeserializer;
use vhr_serde::ser::VHSerializer;

use super::{character_data::*, profile::*};
use crate::common::wrapper::WrapperArray;
use crate::prelude::*;

/// an unknown data version
/// provides a stub so deserialization can fail
/// todo: write (de)serializer to always fail.
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct UnknownVersion;

/// a version of the data structure known to not exist
/// e.g. 0 isnt used as a version as far as i can tell
/// todo: remove this boilerplate with changes to serialization so
/// these impossible versions don't need to be represented here
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct NoSuchVersion;

pub trait Versioned {
    fn to_latest(self) -> Self;
}

/// an enum over all character inventory versions
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum CharacterInventory {
    Zero(NoSuchVersion),

    One(NoSuchVersion),
    Two(NoSuchVersion),
    Three(NoSuchVersion),
    Four(NoSuchVersion),
    Five(NoSuchVersion),
    Six(NoSuchVersion),
    Seven(NoSuchVersion),
    Eight(NoSuchVersion),
    Nine(NoSuchVersion),
    Ten(NoSuchVersion),

    Eleven(NoSuchVersion),
    Twelve(NoSuchVersion),
    Thirteen(NoSuchVersion),
    Fourteen(NoSuchVersion),
    Fifteen(NoSuchVersion),
    Sixteen(NoSuchVersion),
    Seventeen(NoSuchVersion),
    Eighteen(NoSuchVersion),
    NineTeen(NoSuchVersion),

    Twenty(NoSuchVersion),
    TwentyOne(NoSuchVersion),
    TwentyTwo(NoSuchVersion),
    TwentyThree(NoSuchVersion),
    TwentyFour(NoSuchVersion),
    TwentyFive(NoSuchVersion),
    TwentySix(NoSuchVersion),
    TwentySeven(NoSuchVersion),
    TwentyEight(NoSuchVersion),
    TwentyNine(NoSuchVersion),

    Thirty(NoSuchVersion),
    ThirtyOne(NoSuchVersion),
    ThirtyTwo(NoSuchVersion),
    ThirtyThree(NoSuchVersion),
    ThirtyFour(NoSuchVersion),
    ThirtyFive(NoSuchVersion),
    ThirtySix(NoSuchVersion),
    ThirtySeven(NoSuchVersion),
    ThirtyEight(NoSuchVersion),
    ThirtyNine(NoSuchVersion),

    Forty(NoSuchVersion),
    FortyOne(NoSuchVersion),
    FortyTwo(NoSuchVersion),
    FortyThree(NoSuchVersion),
    FortyFour(NoSuchVersion),
    FortyFive(NoSuchVersion),
    FortySix(NoSuchVersion),
    FortySeven(NoSuchVersion),
    FortyEight(NoSuchVersion),
    FortyNine(NoSuchVersion),

    Fifty(NoSuchVersion),
    FiftyOne(NoSuchVersion),
    FiftyTwo(NoSuchVersion),
    FiftyThree(NoSuchVersion),
    FiftyFour(NoSuchVersion),
    FiftyFive(NoSuchVersion),
    FiftySix(NoSuchVersion),
    FiftySeven(NoSuchVersion),
    FiftyEight(NoSuchVersion),
    FiftyNine(NoSuchVersion),

    Sixty(NoSuchVersion),
    SixtyOne(NoSuchVersion),
    SixtyTwo(NoSuchVersion),
    SixtyThree(NoSuchVersion),
    SixtyFour(NoSuchVersion),
    SixtyFive(NoSuchVersion),
    SixtySix(NoSuchVersion),
    SixtySeven(NoSuchVersion),
    SixtyEight(NoSuchVersion),
    SixtyNine(NoSuchVersion),

    Seventy(NoSuchVersion),
    SeventyOne(NoSuchVersion),
    SeventyTwo(NoSuchVersion),
    SeventyThree(NoSuchVersion),
    SeventyFour(NoSuchVersion),
    SeventyFive(NoSuchVersion),
    SeventySix(NoSuchVersion),
    SeventySeven(NoSuchVersion),
    SeventyEight(NoSuchVersion),
    SeventyNine(NoSuchVersion),

    Eighty(NoSuchVersion),
    EightyOne(NoSuchVersion),
    EightyTwo(NoSuchVersion),
    EightyThree(NoSuchVersion),
    EightyFour(NoSuchVersion),
    EightyFive(NoSuchVersion),
    EightySix(NoSuchVersion),
    EightySeven(NoSuchVersion),
    EightyEight(NoSuchVersion),
    EightyNine(NoSuchVersion),

    Ninety(NoSuchVersion),
    NinetyOne(NoSuchVersion),
    NinetyTwo(NoSuchVersion),
    NinetyThree(NoSuchVersion),
    NinetyFour(NoSuchVersion),
    NinetyFive(NoSuchVersion),
    NinetySix(NoSuchVersion),
    NinetySeven(NoSuchVersion),
    NinetyEight(NoSuchVersion),
    NinetyNine(NoSuchVersion),

    OneHundred(Vec<(String, u32, f32, (u32, u32), bool)>),
    OneOhOne(Vec<(String, u32, f32, (u32, u32), bool, u32)>),
    OneOhTwo(Vec<(String, u32, f32, (u32, u32), bool, u32, u32)>),
    OneOhThree(Vec<Item>),
}

impl CharacterInventory {
    pub fn wrap_latest(inner: Vec<Item>) -> CharacterInventory {
        CharacterInventory::OneOhThree(inner)
    }
    pub fn to_latest(self) -> Option<Vec<Item>> {
        use CharacterInventory::*;
        match self {
            OneHundred(_) => None,
            OneOhOne(_) => None,
            OneOhTwo(_) => None,
            OneOhThree(inner) => Some(inner),
            _ => None,
        }
    }
}

/// an enum with all versions of the player's profile data type
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum PlayerProfile {
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
    TwentySeven(Profile),
    TwentyEight(Profile),
    TwentyNine(Profile),
    Thirty(Profile),
    ThirtyOne(Profile),
    ThirtyTwo(Profile),
    ThirtyThree(Profile),
    ThirtyFour(Profile),
    ThirtyFive(Profile),
    ThirtySix(Profile),
}

impl PlayerProfile {
    pub fn to_inner(self) -> Option<Profile> {
        use PlayerProfile::*;
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

        // ThirtySix(p)
    }
    pub fn wrap_latest(inner: Profile) -> Self {
        PlayerProfile::ThirtySix(inner)
    }
}

/// an enum with player skills versions
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum CharacterSkills {
    Zero(NoSuchVersion),
    One(Vec<(u32, f32)>),
    Two(Vec<Skill>),
}

impl CharacterSkills {
    pub fn new() -> Self {
        CharacterSkills::Two(Vec::new())
    }
    pub fn wrap_latest(inner: Vec<Skill>) -> CharacterSkills {
        CharacterSkills::Two(inner)
    }
    pub fn to_latest(self) -> Option<Vec<Skill>> {
        use CharacterSkills::*;
        match self {
            Zero(_) => None,
            One(_inner) => None, // todo: convert these.
            Two(inner) => Some(inner),
        }
    }
}

/// an enum with all versions of the player's inner profile data type
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum Player {
    Zero(NoSuchVersion),
    One(PlayerOne),
    Two(PlayerTwo),
    Three(PlayerThree),
    Four(PlayerFour),
    Five(PlayerFive),
    Six(PlayerSix),
    Seven(PlayerSeven),
    Eight(PlayerEight),
    Nine(PlayerNine),
    Ten(PlayerTen),
    Eleven(PlayerEleven),
    Twelve(PlayerTwelve),
    Thirteen(PlayerThirteen),
    Fourteen(PlayerFourteen),
    Fifteen(PlayerFifteen),
    Sixteen(PlayerSixteen),
    Seventeen(PlayerSeventeen),
    Eighteen(PlayerEighteen),
    Nineteen(PlayerNineteen),
    Twenty(PlayerTwenty),
    TwentyOne(PlayerTwentyOne),
    TwentyTwo(PlayerTwentyTwo),
    TwentyThree(PlayerTwentyThree),
    TwentyFour(PlayerTwentyFour),
    TwentyFive(PlayerTwentyFive),
}

pub type LatestPlayer = PlayerTwentyFive;
impl Player {
    pub fn wrap_latest(inner: PlayerTwentyFive) -> Player {
        Player::TwentyFive(inner)
    }
    pub fn to_latest(self) -> LatestPlayer {
        use Player::*;
        match self {
            Zero(_) => panic!("shouldnt occur"),
            One(inner) => inner
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade(),
            Two(inner) => inner
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade(),
            Three(inner) => inner
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade(),
            Four(inner) => inner
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade(),
            Five(inner) => inner
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade(),
            Six(inner) => inner
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade(),
            Seven(inner) => inner
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade(),
            Eight(inner) => inner
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade(),
            Nine(inner) => inner
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade(),
            Ten(inner) => inner
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade(),
            Eleven(inner) => inner
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade(),
            Twelve(inner) => inner
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade(),
            Thirteen(inner) => inner
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade(),
            Fourteen(inner) => inner
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade(),
            Fifteen(inner) => inner
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade(),
            Sixteen(inner) => inner
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade(),
            Seventeen(inner) => inner
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade(),
            Eighteen(inner) => inner
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade(),
            Nineteen(inner) => inner
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade()
                .upgrade(),
            Twenty(inner) => inner.upgrade().upgrade().upgrade().upgrade().upgrade(),
            TwentyOne(inner) => inner.upgrade().upgrade().upgrade().upgrade(),
            TwentyTwo(inner) => inner.upgrade().upgrade().upgrade(),
            TwentyThree(inner) => inner.upgrade().upgrade(),
            TwentyFour(inner) => inner.upgrade(),
            TwentyFive(inner) => inner,
        }
    }
}

impl Wrapped for Player {
    fn strip(wrapper: WrapperArray) -> Wrapper<Player> {
        let mut deserializer = VHDeserializer::from_owned(wrapper.inner, ());
        let inner = <Player as Deserialize>::deserialize(&mut deserializer).unwrap();
        Wrapper { inner }
    }
    fn wrap(item: Wrapper<Player>) -> WrapperArray {
        let inner = {
            let mut serializer = VHSerializer::new();
            <Player as Serialize>::serialize(&item.inner, &mut serializer).unwrap();
            serializer.to_inner()
        };
        WrapperArray { inner }
    }
}
