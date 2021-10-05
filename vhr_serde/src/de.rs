use std::fs::File;

use crate::error::{Error, Result};
use serde::{
    de::{
        self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess,
        Visitor,
    },
    Deserialize,
};

pub struct VHDeserializer<'de> {
    input: &'de [u8],
    index: usize,
}

impl<'de> VHDeserializer<'de> {
    pub fn from_bytes(input: &'de [u8]) -> Self {
        VHDeserializer { input, index: 0 }
    }
    fn increment(&mut self) -> Result<()> {
        self.index = self.index.checked_add(1).ok_or(Error::Other)?; // buffer larger than usize?
        Ok(())
    }
    fn take_byte(&mut self) -> Result<u8> {
        let value = self
            .input
            .get(self.index)
            .ok_or(Error::ReachedUnexpectedEnd)?;
        self.increment()?;
        Ok(*value)
    }
    fn take_u8(&mut self) -> Result<u8> {
        let value = self
            .input
            .get(self.index)
            .ok_or(Error::ReachedUnexpectedEnd)?;
        self.increment()?;
        // println!("got u8: {}", value);
        Ok(*value)
    }
    fn take_varint(&mut self) -> Result<usize> {
        let first = self.take_byte()? as usize;
        let mut len = first;
        if first > 127 {
            // ie if high bit is set
            let second = self.take_byte()? as usize;
            len += (second - 1) * 128;
        }
        // println!("got varint: {}", len);
        Ok(len)
    }
    fn take_bool(&mut self) -> Result<bool> {
        let value = self.take_byte()?;
        let b = if value == 0 { false } else { true };
        // println!("got bool: {}", b);
        Ok(b)
    }
    /// takes an ascii character from the buffer. clears high bit.
    fn take_char(&mut self) -> Result<char> {
        let mut value = self.take_byte()?;
        value &= 0b01111111; // clear high bit
        let c = char::from_u32(value as u32).ok_or(Error::Other)?;
        Ok(c)
    }
    fn take_string(&mut self) -> Result<String> {
        let len = self.take_byte()?;
        let mut s = String::with_capacity(len as usize);
        for _ in 0..len {
            s.push(self.take_char()?);
        }

        Ok(s)
    }
    fn take_i32(&mut self) -> Result<i32> {
        let bytes: [u8; 4] = [
            self.take_byte()?,
            self.take_byte()?,
            self.take_byte()?,
            self.take_byte()?,
        ];
        let num = i32::from_le_bytes(bytes);
        // println!(
        //     "got i32: {} {} {} {} = {}",
        //     bytes[0], bytes[1], bytes[2], bytes[3], num
        // );
        Ok(num)
    }
    fn take_u32(&mut self) -> Result<u32> {
        let bytes: [u8; 4] = [
            self.take_byte()?,
            self.take_byte()?,
            self.take_byte()?,
            self.take_byte()?,
        ];
        let num = u32::from_le_bytes(bytes);
        // println!(
        //     "got u32: {} {} {} {} = {}",
        //     bytes[0], bytes[1], bytes[2], bytes[3], num
        // );
        Ok(num)
    }
    fn take_f32(&mut self) -> Result<f32> {
        let bytes: [u8; 4] = [
            self.take_byte()?,
            self.take_byte()?,
            self.take_byte()?,
            self.take_byte()?,
        ];
        let num = f32::from_le_bytes(bytes);
        // println!("got f32: {}", num);
        Ok(num)
    }
    fn take_u64(&mut self) -> Result<u64> {
        let bytes: [u8; 8] = [
            self.take_byte()?,
            self.take_byte()?,
            self.take_byte()?,
            self.take_byte()?,
            self.take_byte()?,
            self.take_byte()?,
            self.take_byte()?,
            self.take_byte()?,
        ];
        let num = u64::from_le_bytes(bytes);
        // println!("got u64: {}", num);
        Ok(num)
    }
}

pub fn from_bytes<'a, T>(s: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = VHDeserializer::from_bytes(s);
    let t = T::deserialize(&mut deserializer)?;
    let remaining = deserializer.input.len() - deserializer.index;
    if remaining == 0 {
        Ok(t)
    } else {
        // // println!("{} bytes remaining.", remaining);
        Err(Error::UnconsumedData)
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut VHDeserializer<'de> {
    type Error = Error;
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::WontImplement)
    }

    // Uses the `parse_bool` parsing function defined above to read the JSON
    // identifier `true` or `false` from the input.
    //
    // Parsing refers to looking at the input and deciding that it contains the
    // JSON value `true` or `false`.
    //
    // Deserialization refers to mapping that JSON value into Serde's data
    // model by invoking one of the `Visitor` methods. In the case of JSON and
    // bool that mapping is straightforward so the distinction may seem silly,
    // but in other cases Deserializers sometimes perform non-obvious mappings.
    // For example the TOML format has a Datetime type and Serde's data model
    // does not. In the `toml` crate, a Datetime in the input is deserialized by
    // mapping it to a Serde data model "struct" type with a special name and a
    // single field containing the Datetime represented as a string.
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.take_bool()?;
        visitor.visit_bool(value)
    }

    // The `parse_signed` function is generic over the integer type `T` so here
    // it is invoked with `T=i8`. The next 8 methods are similar.
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // visitor.visit_i8(self.parse_signed()?)
        Err(Error::NotYetImplemented)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // visitor.visit_i16(self.parse_signed()?)
        Err(Error::NotYetImplemented)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let num = self.take_i32()?;
        visitor.visit_i32(num)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // visitor.visit_i64(self.parse_signed()?)
        unimplemented!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.take_u8()?;
        visitor.visit_u8(value)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // visitor.visit_u16(self.parse_unsigned()?)
        unimplemented!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.take_u32()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let num = self.take_u64()?;
        visitor.visit_u64(num)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let num = self.take_f32()?;
        visitor.visit_f32(num)
    }

    // Float parsing is stupidly hard.
    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // The `Serializer` implementation on the previous page serialized chars as
    // single-character strings so handle that representation here.
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Parse a string, check that it is one character, call `visit_char`.
        let ch = self.take_char()?;
        visitor.visit_char(ch)
    }

    // Refer to the "Understanding deserializer lifetimes" page for information
    // about the three deserialization flavors of strings in Serde.
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // visitor.visit_borrowed_str(self.parse_string()?)
        Err(Error::NotYetImplemented)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let s = self.take_string()?;
        visitor.visit_str(&s)
    }

    // The `Serializer` implementation on the previous page serialized byte
    // arrays as JSON arrays of bytes. Handle that representation here.
    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // An absent optional is represented as the JSON `null` and a present
    // optional is represented as just the contained value.
    //
    // As commented in `Serializer` implementation, this is a lossy
    // representation. For example the values `Some(())` and `None` both
    // serialize as just `null`. Unfortunately this is typically what people
    // expect when working with JSON. Other formats are encouraged to behave
    // more intelligently if possible.
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.take_bool()? {
            visitor.visit_some(self)
        } else {
            visitor.visit_none()
        }
    }

    // In Serde, unit means an anonymous value containing no data.
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // if self.input.starts_with("null") {
        //     self.input = &self.input["null".len()..];
        //     visitor.visit_unit()
        // } else {
        //     Err(Error::ExpectedNull)
        // }
        unimplemented!()
    }

    // Unit struct means a named value containing no data.
    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain. That means not
    // parsing anything other than the contained value.
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    // Deserialization of compound types like sequences and maps happens by
    // passing the visitor an "Access" object that gives it the ability to
    // iterate through the data contained in the sequence.
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Parse the opening bracket of the sequence.
        // if self.next_char()? == '[' {
        //     // Give the visitor access to each element of the sequence.
        //     let value = visitor.visit_seq(CommaSeparated::new(self))?;
        //     // Parse the closing bracket of the sequence.
        //     if self.next_char()? == ']' {
        //         Ok(value)
        //     } else {
        //         Err(Error::ExpectedArrayEnd)
        //     }
        // } else {
        //     Err(Error::ExpectedArray)
        // }
        let len = self.take_i32()? as usize;
        // println!("Sequence: ({})", len);
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(TupleAccess {
            deserializer: self,
            len,
        })
    }

    // Tuple structs look just like sequences in JSON.
    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // println!("{} ({})", name, len);
        self.deserialize_seq(visitor)
    }

    // Much like `deserialize_seq` but calls the visitors `visit_map` method
    // with a `MapAccess` implementation, rather than the visitor's `visit_seq`
    // method with a `SeqAccess` implementation.
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = self.take_varint()?;
        let v = visitor.visit_map(ShortMapAccess {
            deserializer: self,
            len,
        });
        v
        // Parse the opening brace of the map.
        // if self.next_char()? == '{' {
        //     // Give the visitor access to each entry of the map.
        //     let value = visitor.visit_map(CommaSeparated::new(self))?;
        //     // Parse the closing brace of the map.
        //     if self.next_char()? == '}' {
        //         Ok(value)
        //     } else {
        //         Err(Error::ExpectedMapEnd)
        //     }
        // } else {
        //     Err(Error::ExpectedMap)
        // }
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // println!("{} ({})", name, fields.len());
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // if self.peek_char()? == '"' {
        //     // Visit a unit variant.
        //     visitor.visit_enum(self.parse_string()?.into_deserializer())
        // } else if self.next_char()? == '{' {
        //     // Visit a newtype variant, tuple variant, or struct variant.
        //     let value = visitor.visit_enum(Enum::new(self))?;
        //     // Parse the matching close brace.
        //     if self.next_char()? == '}' {
        //         Ok(value)
        //     } else {
        //         Err(Error::ExpectedMapEnd)
        //     }
        // } else {
        //     Err(Error::ExpectedEnum)
        // }
        unimplemented!()
    }

    // An identifier in Serde is the type that identifies a field of a struct or
    // the variant of an enum. In JSON, struct fields and enum variants are
    // represented as strings. In other formats they may be represented as
    // numeric indices.
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // self.deserialize_str(visitor)
        unimplemented!()
    }

    // Like `deserialize_any` but indicates to the `Deserializer` that it makes
    // no difference which `Visitor` method is called because the data is
    // ignored.
    //
    // Some deserializers are able to implement this more efficiently than
    // `deserialize_any`, for example by rapidly skipping over matched
    // delimiters without paying close attention to the data in between.
    //
    // Some formats are not able to implement this at all. Formats that can
    // implement `deserialize_any` and `deserialize_ignored_any` are known as
    // self-describing.
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // self.deserialize_any(visitor)
        unimplemented!()
    }
}

struct ShortMapAccess<'a, 'de: 'a> {
    deserializer: &'a mut VHDeserializer<'de>,
    len: usize,
}

impl<'de, 'a> MapAccess<'de> for ShortMapAccess<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        // Check if there are no more entries.
        if self.len == 0 {
            Ok(None)
        } else {
            self.len -= 1;
            seed.deserialize(&mut *self.deserializer).map(Some)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.deserializer)
    }
}

struct TupleAccess<'a, 'de: 'a> {
    deserializer: &'a mut VHDeserializer<'de>,
    len: usize,
}

impl<'de, 'a> SeqAccess<'de> for TupleAccess<'a, 'de> {
    type Error = Error;
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.len > 0 {
            self.len -= 1;
            let value = serde::de::DeserializeSeed::deserialize(seed, &mut *self.deserializer)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

struct Unlabled<'a, 'de: 'a> {
    de: &'a mut VHDeserializer<'de>,
}

impl<'de, 'a> MapAccess<'de> for Unlabled<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        // // Check if there are no more entries.
        // if self.de.peek_char()? == '}' {
        //     return Ok(None);
        // }
        // // Comma is required before every entry except the first.
        // if !self.first && self.de.next_char()? != ',' {
        //     return Err(Error::ExpectedMapComma);
        // }
        // self.first = false;
        // // Deserialize a map key.
        // seed.deserialize(&mut *self.de).map(Some)
        unimplemented!()
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        // // It doesn't make a difference whether the colon is parsed at the end
        // // of `next_key_seed` or at the beginning of `next_value_seed`. In this
        // // case the code is a bit simpler having it here.
        // if self.de.next_char()? != ':' {
        //     return Err(Error::ExpectedMapColon);
        // }
        // // Deserialize a map value.
        // seed.deserialize(&mut *self.de)
        unimplemented!()
    }
}
