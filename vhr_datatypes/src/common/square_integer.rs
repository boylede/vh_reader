use serde::{
    de::{DeserializeSeed, Error, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

// #[derive(Default, Clone, PartialEq, Debug, Serialize)]
// pub struct SquareInt(u32);

// impl SquareInt {
//     pub fn from_u32(integer: u32) -> SquareInt {
//         SquareInt(integer)
//     }
//     pub fn to_inner(self) -> u32 {
//         self.0
//     }
// }

// impl<'de> Deserialize<'de> for SquareInt {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         deserializer.deserialize_i32(SquareIntVisitor)
//     }
// }

// struct SquareIntVisitor;

// impl<'de> Visitor<'de> for SquareIntVisitor {
//     type Value = SquareInt;

//     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         formatter.write_str("some number that we'll square before returning")
//     }

//     fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
//     where
//         E: Error,
//     {
//         Ok(SquareInt(value * value))
//     }
// }
