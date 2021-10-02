use druid::Data;

#[derive(PartialEq, Eq, Data, Clone)]
#[non_exhaustive]
pub enum HairType {
    None,
    Hair1,
    Hair2,
    Hair3,
    Hair4,
    Hair5,
    Hair6,
    Hair7,
    Hair8,
    Hair9,
    Hair10,
    Hair11,
    Hair12,
    Hair13,
    Hair14,
    Other(String),
}

// pub struct HairLens;

// impl Lens<String, HairType> for HairLens {
//     fn with<V, F: FnOnce(&HairType) -> V>(&self, data: &String, f: F) -> V {
//         // if builtin_hair_names.contains(&data.as_str()) {
//         for (mat, variant) in hair_radio_group.iter() {
//             if *mat == data.as_str() {
//                 return f(&variant);
//             }
//         }
//         return f(&HairType::Other(data.clone()))
//         // } else {
//         // panic!("couldn't find string {}", data);
//         // }

//     }
//     fn with_mut<V, F: FnOnce(&mut HairType) -> V>(&self, data: &mut String, f: F) -> V {
//         let mut hair_type: HairType = {
//             let mut found = None;
//             for (mat, variant) in hair_radio_group.iter() {
//                 if *mat == data.as_str() {
//                     found = Some(variant);
//                     break;
//                 }
//             }
//             if let Some(v) = found {
//                 v.clone()
//             } else {
//                 HairType::Other(data.clone())
//                 // panic!("couldnt find variant");
//             }
//         };
//         let value = f(&mut hair_type);
//         //todo actually set backing value
//         value
//     }
// }

// Then::new(
//     LoadedCharacter::kill_count,
//     Map::new(
//         |get: &u32| get.to_string(),
//         |set: &mut u32, value: String| *set = value.parse().unwrap_or(0),
//     )

// pub const hair_radio_group: &[(&str, HairType)] = &[
//     ("No Hair", HairType::None),
//     ("Braided 1", HairType::Hair1),
//     ("Braided 2", HairType::Hair2),
//     ("Braided 3", HairType::Hair3),
//     ("Braided 4", HairType::Hair4),
//     ("Long 1", HairType::Hair5),
//     ("Ponytail 1", HairType::Hair6),
//     ("Ponytail 2", HairType::Hair7),
//     ("Ponytail 3", HairType::Hair8),
//     ("Ponytail 4", HairType::Hair9),
//     ("Short 1", HairType::Hair10),
//     ("Short 2", HairType::Hair11),
//     ("Side Swept 1", HairType::Hair12),
//     ("Side Swept 2", HairType::Hair13),
//     ("Side Swept 3", HairType::Hair14),
// ];

// const builtin_hair_names: &[&str] = &[
//     "No Hair",
//     "Braided 1",
//     "Braided 2",
//     "Braided 3",
//     "Braided 4",
//     "Long 1",
//     "Ponytail 1",
//     "Ponytail 2",
//     "Ponytail 3",
//     "Ponytail 4",
//     "Short 1",
//     "Short 2",
//     "Side Swept 1",
//     "Side Swept 2",
//     "Side Swept 3",
// ];

// const builtin_beard_names: &[&str] = &[
//     "No Beard",
//     "Braided 1",
//     "Braided 2",
//     "Braided 3",
//     "Braided 4",
//     "Long 1",
//     "Long 2",
//     "Short 1",
//     "Short 2",
//     "Short 3",
//     "Thick 1",
// ];
