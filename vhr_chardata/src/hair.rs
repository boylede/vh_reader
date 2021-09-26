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

