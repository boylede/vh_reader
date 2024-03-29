use crate::{
    error::{Error, Result},
};
use serde::{
    ser::{self, Serializer},
    Serialize,
};

pub trait SerializeOptions {
    fn modify_sequence_length(&mut self, length: usize) -> Option<usize> {
        Some(length)
    }
    fn omit_sequence_length(&mut self) -> bool {
        false
    }
}

impl SerializeOptions for () {}

#[derive(Default)]
pub struct VHSerializer<O> {
    output: Vec<u8>,
    options: O,
}

// impl SerializeOptions for () {}

impl VHSerializer<()> {
    pub fn new() -> Self {
        let options = ();
        VHSerializer {
            output: Vec::new(),
            options,
        }
    }
}

impl<O> VHSerializer<O>
where
    O: SerializeOptions,
{
    pub fn with_options(options: O) -> VHSerializer<O>
    where
        O: SerializeOptions,
    {
        VHSerializer {
            output: Vec::new(),
            options,
        }
    }
    pub fn to_inner(self) -> Vec<u8> {
        self.output
    }
    fn push_bool(&mut self, v: bool) -> Result<()> {
        self.output.push(if v { 1 } else { 0 });
        Ok(())
    }
    fn push_u8(&mut self, v: u8) -> Result<()> {
        self.output.push(v);
        Ok(())
    }
    fn push_varint(&mut self, v: usize) -> Result<()> {
        // 32 bit max int width, 7 bits per loop = 5 loops
        // before we;ve gotten all possible bytes
        let mut integer: u32 = v as u32;
        const LO_BITS: u32 = (1 << 7) - 1;
        const HI_BITS: u32 = !LO_BITS;
        for _ in 0..5u32 {
            let byte = (integer & LO_BITS) as u8;
            if integer & HI_BITS == 0 {
                self.push_u8(byte)?;
                break;
            } else {
                self.push_u8(byte | 0x80)?;
                integer >>= 7;
                // if high bit set
            }
        }
        Ok(())
    }
    fn push_i32(&mut self, v: i32) -> Result<()> {
        for b in v.to_le_bytes() {
            self.output.push(b);
        }
        Ok(())
    }
    fn push_char(&mut self, v: char) -> Result<()> {
        if v.is_ascii() {
            self.output.push(v as u8);
            Ok(())
        } else {
            Err(Error::NonAsciiString)
        }
    }
}

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut serializer = VHSerializer::new();
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a, O> Serializer for &'a mut VHSerializer<O>
where
    O: SerializeOptions,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.push_bool(v)
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.output.push(v as u8);
        Ok(())
    }

    fn serialize_i16(self, _v: i16) -> Result<()> {
        // Err(Error::WontImplement)
        unimplemented!()
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.push_i32(v)
    }

    fn serialize_i64(self, _v: i64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.output.push(v);
        Ok(())
    }

    fn serialize_u16(self, _v: u16) -> Result<()> {
        // Err(Error::WontImplement)
        unimplemented!()
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        // println!("serializing {}", v);
        self.output.extend_from_slice(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output.extend_from_slice(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.output.extend_from_slice(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_f64(self, _v: f64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_char(self, v: char) -> Result<()> {
        // Err(Error::WontImplement)
        self.push_char(v)
    }
    fn serialize_str(self, v: &str) -> Result<()> {
        let original_len = v.len();
        // if original_len > 255 {
        //     println!("str longer than 255");
        //     return Err(Error::OverlargeData);
        // } else {
        self.push_varint(original_len)?;
        let mut new_string: Vec<u8> = v
            .chars()
            .filter(|c| c.is_ascii())
            .map(|c| c as u8)
            .collect();
        if new_string.len() < original_len {
            return Err(Error::NonAsciiString);
        } else {
            self.output.append(&mut new_string);
            Ok(())
        }
        // }
    }

    // use short form here
    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<()> {
        self.push_bool(false)
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.push_bool(true)?;
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    // do we need this?
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        // unimplemented!()
        println!(
            "serializing unit variant {}, {}, {}",
            name, variant_index, variant
        );
        Ok(())
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_u32(variant_index)?;
        value.serialize(self)
    }
    // use long form length here
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        if let Some(len) = len {
            // todo: error on too large
            let len = self.options.modify_sequence_length(len);
            if let Some(len) = len {
                self.serialize_u32(len as u32)?;
            }
            Ok(self)
        } else {
            Err(Error::UnknownSize)
        }
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }
    // use short form length here
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        if let Some(len) = len {
            // todo: error on too large
            if len > 255 {
                println!("map longer than 255");
                Err(Error::OverlargeData)
            } else {
                self.push_u8(len as u8)?;
                Ok(self)
            }
        } else {
            Err(Error::UnknownSize)
        }
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
        // unimplemented!()
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }
}

impl<'a, O> ser::SerializeSeq for &'a mut VHSerializer<O>
where
    O: SerializeOptions,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, O> ser::SerializeTuple for &'a mut VHSerializer<O>
where
    O: SerializeOptions,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, O> ser::SerializeTupleStruct for &'a mut VHSerializer<O>
where
    O: SerializeOptions,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, O> ser::SerializeTupleVariant for &'a mut VHSerializer<O>
where
    O: SerializeOptions,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, O> ser::SerializeMap for &'a mut VHSerializer<O>
where
    O: SerializeOptions,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, O> ser::SerializeStruct for &'a mut VHSerializer<O>
where
    O: SerializeOptions,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, O> ser::SerializeStructVariant for &'a mut VHSerializer<O>
where
    O: SerializeOptions,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

#[test]
fn test_varint_set() {
    let mut serializer = VHSerializer::new();
    serializer.push_varint(1344).expect("failed to serialzie");
    let out = serializer.to_inner();
    let expected = vec![192, 10];
    assert_eq!(out, expected, "we expected {:?} == {:?}", out, expected);
}

#[test]
fn test_varint_set_low() {
    let mut serializer = VHSerializer::new();
    serializer.push_varint(28).expect("failed to serialzie");
    let out = serializer.to_inner();
    let expected = vec![28];
    assert_eq!(out, expected, "we expected {:?} == {:?}", out, expected);
}
