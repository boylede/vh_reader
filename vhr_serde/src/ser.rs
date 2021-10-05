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
    fn push_bool(&mut self, v: bool) -> Result<()> {
        self.output.push(if v { 1 } else { 0 });
        Ok(())
    }
    // todo: check implementation
    fn push_varint(&mut self, v: usize) -> Result<()> {
        // todo: we're assuming two-bytes max because thats all we've seen
        // let first = self.take_byte()? as usize;
        // let mut len = first;
        // if first > 127 { // ie if high bit is set
        //     let second = self.take_byte()? as usize;
        //     len += (second - 1) * 128;
        // }
        // println!("got varint: {}", len);
        // Ok(len)

        if v > 127 {
            let first = v - 127;
            // we're assuming that "first" is now a value between 128 and 255
            // i.e. the high bit is set in an 8-bit integer
            self.output.push(first as u8);
            // now we need to push the remainder
            let second = ((v - first) / 128) - 1;
            self.output.push(second as u8);
            Ok(())
        } else {
            self.output.push(v as u8);
            Ok(())
        }
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
        self.push_bool(v)
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.output.push(v as u8);
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        // Err(Error::WontImplement)
        unimplemented!()
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.push_i32(v)
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.output.push(v);
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
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

    fn serialize_f64(self, v: f64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_char(self, v: char) -> Result<()> {
        // Err(Error::WontImplement)
        self.push_char(v)
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
        unimplemented!()
    }

    fn serialize_none(self) -> Result<()> {
        self.push_bool(false)

    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {   
        self.push_bool(true);
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
        unimplemented!()
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
        unimplemented!()
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
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
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
        // unimplemented!()
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
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
