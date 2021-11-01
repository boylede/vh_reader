use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::wrappers::versioning::{NoSuchVersion, UnknownVersion};

/// an enum over all character inventory versions
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum InventoryVersions {
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

impl InventoryVersions {
    pub fn wrap_latest(inner: Vec<Item>) -> InventoryVersions {
        InventoryVersions::OneOhThree(inner)
    }
    pub fn to_latest(self) -> Option<Vec<Item>> {
        use InventoryVersions::*;
        match self {
            OneHundred(_) => None,
            OneOhOne(_) => None,
            OneOhTwo(_) => None,
            OneOhThree(inner) => Some(inner),
            _ => None,
        }
    }
}
