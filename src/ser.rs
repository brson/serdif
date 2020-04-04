#![allow(unused)]

use serde::{ser, Serialize};

use crate::error::{Error, Result, ResultExt};
use crate::scmd;
use crate::state::{State, Buffer};
use std::io::SeekFrom;

use crate::de::Deserializer;

pub struct Serializer {
    state: State,
}

impl Serializer {
    pub fn new(buf: impl Buffer) -> Serializer {
        Serializer::from_state(State {
            buf: Box::new(buf),
        })
    }

    pub fn from_state(state: State) -> Serializer {
        Serializer { state }
    }

    pub fn to_state(self) -> State {
        self.state
    }

    pub fn to_de(self) -> Deserializer {
        Deserializer::from_state(self.to_state())
    }

    pub fn reset(&mut self) -> Result<()> {
        self.state.buf.seek(SeekFrom::Start(0)).e()?;
        Ok(())
    }

    fn write(&mut self, v: impl Serialize) -> Result<()> {
        Ok(serde_json::to_writer_pretty(&mut self.state.buf, &v).e()?)
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
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
        Ok(self.write(scmd::SerializeBool { v })?)
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        panic!()
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        panic!()
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        panic!()
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        panic!()
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        Ok(self.write(scmd::SerializeU8 { v })?)
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        panic!()
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        panic!()
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        panic!()
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        panic!()
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        panic!()
    }

    fn serialize_char(self, v: char) -> Result<()> {
        panic!()
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        panic!()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        panic!()
    }

    fn serialize_none(self) -> Result<()> {
        panic!()
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        panic!()
    }

    fn serialize_unit(self) -> Result<()> {
        panic!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        panic!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        panic!()
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        panic!()
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
        panic!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        panic!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        panic!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        panic!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        panic!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        panic!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        self.write(scmd::SerializeStruct {
            name, len
        })?;
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        panic!()
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        panic!()
    }

    fn end(self) -> Result<()> {
        panic!()
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        panic!()
    }

    fn end(self) -> Result<()> {
        panic!()
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        panic!()
    }

    fn end(self) -> Result<()> {
        panic!()
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        panic!()
    }

    fn end(self) -> Result<()> {
        panic!()
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        panic!()
    }

    // It doesn't make a difference whether the colon is printed at the end of
    // `serialize_key` or at the beginning of `serialize_value`. In this case
    // the code is a bit simpler having it here.
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        panic!()
    }

    fn end(self) -> Result<()> {
        panic!()
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.write(scmd::SerializeField {
            key, value
        })?;
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.write(scmd::SerializeStructEnd)?;
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        panic!()
    }

    fn end(self) -> Result<()> {
        panic!()
    }
}

