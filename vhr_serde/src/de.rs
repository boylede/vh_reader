use std::{fs::File, marker::PhantomData};

use crate::error::{Error, Result};
use serde::{
    de::{
        self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess,
        Visitor,
    },
    Deserialize,
};

pub trait DeserializeOptions {
    fn modify_sequence_length(&mut self, length: usize) -> usize {
        length
    }
}

impl DeserializeOptions for () { }

pub struct VHDeserializer<'de, O> {
    input: &'de [u8],
    index: usize,
    options: O,
}

impl<'de> VHDeserializer<'de, ()> {
    pub fn from_bytes(input: &'de [u8]) -> VHDeserializer<'de, ()> {
        VHDeserializer { input, index: 0, options: ()}
    }
}

impl<'de, O> VHDeserializer<'de, O> {
    pub fn from_bytes_options(input: &'de [u8], options: O) -> VHDeserializer<'de, O> {
        VHDeserializer { input, index: 0, options}
    }
    pub fn into_inner(self) -> &'de [u8] {
        self.input
    }
    fn increment(&mut self) -> Result<()> {
        self.index = self.index.checked_add(1).ok_or(Error::IndexOverflowed)?; // buffer larger than usize?
        Ok(())
    }
    fn peek_byte(&self, offset: usize) -> Result<u8> {
        let index = self.index.checked_add(offset).ok_or_else(||Error::ReachedUnexpectedEnd)?;
        self.input.get(index).copied().ok_or_else(|| {
            Error::ReachedUnexpectedEnd
        })
    }
    pub fn peek_u32(&mut self) -> Result<u32> {
        let bytes: [u8; 4] = [
            self.peek_byte(0)?,
            self.peek_byte(1)?,
            self.peek_byte(2)?,
            self.peek_byte(3)?,
        ];
        let num = u32::from_le_bytes(bytes);
        Ok(num)
    }
    fn take_byte(&mut self) -> Result<u8> {
        let value = self.input.get(self.index).ok_or_else(|| {
            // println!("reached end at {} of {}", self.index, self.input.len());
            Error::ReachedUnexpectedEnd
        })?;
        self.increment()?;
        Ok(*value)
    }
    pub fn take_u8(&mut self) -> Result<u8> {
        let value = self.take_byte()?;
        // println!("got u8: {}", value);
        Ok(value)
    }
    // a variable length integer
    // used to prefix strings
    pub fn take_varint(&mut self) -> Result<usize> {
        // 32 bit max int width, 7 bits per loop = 5 loops
        // before we;ve gotten all possible bytes
        let mut integer: u32 = 0;
        for i in 0..5u32 {
            let byte = self.take_byte()?;
            integer |= (byte as u32 & 0x7f) << i * 7;
            // if high bit set
            if byte <= 127 {
                break;
            }
        }
        Ok(integer as usize)
    }

    pub fn take_bool(&mut self) -> Result<bool> {
        let value = self.take_byte()?;
        let b = if value == 0 { false } else { true };
        // println!("got bool: {}", b);
        Ok(b)
    }
    /// takes an ascii character from the buffer. clears high bit.
    pub fn take_char(&mut self) -> Result<char> {
        let mut value = self.take_byte()?;
        value &= 0b01111111; // clear high bit
        let c = char::from_u32(value as u32).ok_or(Error::CharacterEncoding)?;
        Ok(c)
    }
    pub fn take_string(&mut self) -> Result<String> {
        let len = self.take_varint()?;
        let mut s = String::with_capacity(len);
        for _ in 0..len {
            s.push(self.take_char()?);
        }

        Ok(s)
    }
    pub fn take_i32(&mut self) -> Result<i32> {
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
    pub fn take_u32(&mut self) -> Result<u32> {
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
    pub fn take_f32(&mut self) -> Result<f32> {
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
    pub fn take_u64(&mut self) -> Result<u64> {
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
    // pub fn take_vec<T>(&mut self) -> Result<Vec<T>> where T: Deserialize<'de> {
    //     let length = self.take_u32()?;
    //     self.take_slice(length as usize)
    // }
    pub fn take_byte_slice(&mut self, length: usize) -> Result<Vec<u8>> {
        let mut v = Vec::with_capacity(length);
        for _ in 0..length {
            v.push(self.take_u8()?);
        }
        Ok(v)
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
        // println!("{} bytes remaining.", remaining);
        Err(Error::UnconsumedData)
    }
}

impl<'de, 'a, O> de::Deserializer<'de> for &'a mut VHDeserializer<'de, O> where O: DeserializeOptions {
    type Error = Error;
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::WontImplement)
    }

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

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = self.take_byte()?;
        // println!("des bb len: {}", len);
        let mut buf = Vec::with_capacity(len as usize);
        if len > 0 {
            for _ in 0..len {
                let b = self.take_byte()?;
                buf.push(b);
            }
        }
        visitor.visit_bytes(&buf)
        // unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = self.take_byte()?;
        // println!("des obb len: {}", len);
        let mut buf = Vec::with_capacity(len as usize);
        if len > 0 {
            for _ in 0..len {
                let b = self.take_byte()?;
                buf.push(b);
            }
        }
        visitor.visit_byte_buf(buf)
        // unimplemented!()
    }

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
        let len = O::modify_sequence_length(&mut self.options, len);
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
        // println!("tuple struct {} ({}) @ {:x}", name, len, self.index);
        self.deserialize_seq(visitor)
    }

    // Much like `deserialize_seq` but calls the visitors `visit_map` method
    // with a `MapAccess` implementation, rather than the visitor's `visit_seq`
    // method with a `SeqAccess` implementation.
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = self.take_u8()? as usize;
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
        // println!("struct {} ({})  @ {:x}", name, fields.len(), self.index);
        self.deserialize_tuple(fields.len(), visitor)
    }
    /*
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let variant = self.take_u32()?;
        println!("found enum variant {} for {},{:?}", variant, name, variants);
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
    */
    fn deserialize_enum<V>(
        self,
        enum_name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        // println!("found enum {} with variants: {:?}", enum_name, variants);
        visitor.visit_enum(self)
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

impl<'de, 'a, O> EnumAccess<'de> for &'a mut VHDeserializer<'de, O> where O: DeserializeOptions {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let variant = self.take_u32()?;
        // println!("found enum variant {}", variant);
        let val: Result<_> = seed.deserialize(variant.into_deserializer());
        Ok((val?, self))
    }
}

impl<'de, 'a, O> serde::de::VariantAccess<'de> for &'a mut VHDeserializer<'de, O> where O: DeserializeOptions {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        serde::de::DeserializeSeed::deserialize(seed, self)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_tuple(self, len, visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_tuple(self, fields.len(), visitor)
    }
}

struct ShortMapAccess<'a, 'de: 'a, O> {
    deserializer: &'a mut VHDeserializer<'de, O>,
    len: usize,
}

impl<'de, 'a, O> MapAccess<'de> for ShortMapAccess<'a, 'de, O> where O: DeserializeOptions {
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

struct TupleAccess<'a, 'de: 'a, O> {
    deserializer: &'a mut VHDeserializer<'de, O>,
    len: usize,
}

impl<'de, 'a, O> SeqAccess<'de> for TupleAccess<'a, 'de, O> where O: DeserializeOptions {
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

struct Unlabled<'a, 'de: 'a, O> {
    de: &'a mut VHDeserializer<'de, O>,
}

impl<'de, 'a, O> MapAccess<'de> for Unlabled<'a, 'de, O> {
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
