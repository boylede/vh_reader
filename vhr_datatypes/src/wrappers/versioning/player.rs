use serde::{Deserialize, Serialize};

use vhr_serde::{VHDeserializer, VHSerializer};

use crate::player::*;
use crate::prelude::*;
use crate::wrappers::versioning::{NoSuchVersion, UnknownVersion};

/// an enum with all versions of the player's inner profile data type
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum PlayerVersions {
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
impl PlayerVersions {
    pub fn wrap_latest(inner: PlayerTwentyFive) -> PlayerVersions {
        PlayerVersions::TwentyFive(inner)
    }
    pub fn to_latest(self) -> LatestPlayer {
        use PlayerVersions::*;
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

impl Wrapped for PlayerVersions {
    fn strip(wrapper: WrapperArray) -> Wrapper<PlayerVersions> {
        let mut deserializer = VHDeserializer::from_owned(wrapper.inner, ());
        let inner = <PlayerVersions as Deserialize>::deserialize(&mut deserializer).unwrap();
        Wrapper { inner }
    }
    fn wrap(item: Wrapper<PlayerVersions>) -> WrapperArray {
        let inner = {
            let mut serializer = VHSerializer::new();
            <PlayerVersions as Serialize>::serialize(&item.inner, &mut serializer).unwrap();
            serializer.to_inner()
        };
        WrapperArray { inner }
    }
}
