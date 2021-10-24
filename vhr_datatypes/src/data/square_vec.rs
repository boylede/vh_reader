use std::env;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::marker::PhantomData;

use serde::{
    de::{DeserializeSeed, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

use std::rc::Rc;

#[derive(Debug)]
pub enum Error {
    Other,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        use Error::*;
        write!(
            f,
            "{}",
            match self {
                Other => "other error",
            }
        )
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::Other
    }
}

impl serde::de::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::Other
    }
}

impl serde::ser::StdError for Error {}

#[derive(Default, Clone, PartialEq, Debug, Serialize)]
pub struct SquareVec<T> {
    edge_length: u32,
    inner: Vec<T>,
}

impl<'de, T> Deserialize<'de> for SquareVec<T>
where
    T: Deserialize<'de> + std::fmt::Debug,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let inner = Vec::new();
        deserializer.deserialize_seq(SquareVecVisitor { inner })
        // unimplemented!()

        // let count = u32::deserialize(deserializer)?;
        // let _ = u16::deserialize(deserializer);
        // let mut assets = Vec::with_capacity(count as usize);
        // for _ in 0..count {
        //     let asset = AssetDescriptor::deserialize(deserializer)?;
        //     assets.push(asset);
        // }
        // let padding = [0;2];
        // Ok(AssetIndex {
        //     count, padding, assets
        // })
    }
}

#[derive(Default)]
struct SquareVecVisitor<T> {
    inner: Vec<T>,
    // _phantom: PhantomData<T>,
}

impl<'de, T> Visitor<'de> for SquareVecVisitor<T>
where
    T: Deserialize<'de> + std::fmt::Debug,
{
    type Value = SquareVec<T>;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("square vector")
    }
    fn visit_seq<A>(mut self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        // todo: figure out the error type mapping
        println!("found a square vec with side length: {:?}", seq.size_hint());
        let size = seq.size_hint().ok_or(Error::Other).unwrap();
        let actual_size = size * size;

        self.inner.reserve(actual_size);
        println!("getting {} elements", actual_size);
        for _ in 0..actual_size {
            //todo: again, fix error type mapping from option to A:Error type
            let n: Option<T> = seq.next_element()?;
            if let Some(nn) = n {
                println!("got element {:?}", nn);
                self.inner.push(nn);
            } else {
                println!("couldnt get element");
            }
        }
        let sv = SquareVec {
            edge_length: size as u32,
            inner: self.inner,
        };
        Ok(sv)
    }
}
/*
use std::fmt;
struct Duration {
    secs: u64,
    nanos: u32,
}

impl<'de> Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Secs, Nanos }

        // This part could also be generated independently by:
        //
        //    #[derive(Deserialize)]
        //    #[serde(field_identifier, rename_all = "lowercase")]
        //    enum Field { Secs, Nanos }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`secs` or `nanos`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "secs" => Ok(Field::Secs),
                            "nanos" => Ok(Field::Nanos),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct DurationVisitor;

        impl<'de> Visitor<'de> for DurationVisitor {
            type Value = Duration;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Duration")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Duration, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let secs = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let nanos = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(Duration::new(secs, nanos))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Duration, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut secs = None;
                let mut nanos = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Secs => {
                            if secs.is_some() {
                                return Err(de::Error::duplicate_field("secs"));
                            }
                            secs = Some(map.next_value()?);
                        }
                        Field::Nanos => {
                            if nanos.is_some() {
                                return Err(de::Error::duplicate_field("nanos"));
                            }
                            nanos = Some(map.next_value()?);
                        }
                    }
                }
                let secs = secs.ok_or_else(|| de::Error::missing_field("secs"))?;
                let nanos = nanos.ok_or_else(|| de::Error::missing_field("nanos"))?;
                Ok(Duration::new(secs, nanos))
            }
        }

        const FIELDS: &'static [&'static str] = &["secs", "nanos"];
        deserializer.deserialize_struct("Duration", FIELDS, DurationVisitor)
    }
}
 */
