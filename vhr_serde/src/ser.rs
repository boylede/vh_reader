use crate::error::{Error, Result};
use serde::{
    ser::{self, Serializer},
    Serialize,
};

#[derive(Default)]
pub struct VHSerializer {
    output: Vec<u8>,
}

impl VHSerializer {
    // todo: check implementation
    fn push_varint(&mut self, v: usize) -> Result<()> {
        if v > 255 {
            self.output.push(255 as u8);
            self.output.push((v - 255) as u8);
            Ok(())
        } else {
            self.output.push(v as u8);
            Ok(())
        }
    }
}

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut serializer = VHSerializer { output: Vec::new() };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> Serializer for &'a mut VHSerializer {
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
        self.output.push(if v { 0 } else { 1 });
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.output.push(v as u8);
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        Err(Error::WontImplement)
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        Err(Error::NotYetImplemented)
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        Err(Error::NotYetImplemented)
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.output.push(v);
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        Err(Error::WontImplement)
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
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

    fn serialize_f64(self, v: f64) -> Result<()> {
        Err(Error::NotYetImplemented)
    }

    fn serialize_char(self, v: char) -> Result<()> {
        Err(Error::WontImplement)
    }
    fn serialize_str(self, v: &str) -> Result<()> {
        let original_len = v.len();
        if original_len > 255 {
            return Err(Error::OverlargeData);
        } else {
            self.serialize_u8(original_len as u8)?;
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
        }
    }

    // use short form here
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        Err(Error::NotYetImplemented)
    }

    fn serialize_none(self) -> Result<()> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
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
        Err(Error::NotYetImplemented)
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
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::NotYetImplemented)
    }
    // use long form length here
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        if let Some(len) = len {
            // todo: error on too large
            self.serialize_u32(len as u32)?;
            Ok(self)
        } else {
            Err(Error::Other)
        }
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::NotYetImplemented)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::NotYetImplemented)
    }
    // use short form length here
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        if let Some(len) = len {
            // todo: error on too large
            self.push_varint(len)?;
            Ok(self)
        } else {
            Err(Error::Other)
        }
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
        // Err(Error::NotYetImplemented)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::NotYetImplemented)
    }
}

impl<'a> ser::SerializeSeq for &'a mut VHSerializer {
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

impl<'a> ser::SerializeTuple for &'a mut VHSerializer {
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

impl<'a> ser::SerializeTupleStruct for &'a mut VHSerializer {
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

impl<'a> ser::SerializeTupleVariant for &'a mut VHSerializer {
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

impl<'a> ser::SerializeMap for &'a mut VHSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
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

impl<'a> ser::SerializeStruct for &'a mut VHSerializer {
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

impl<'a> ser::SerializeStructVariant for &'a mut VHSerializer {
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
