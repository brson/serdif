#![allow(unused)]

use serde::{ser, Serialize};

use crate::error::{Error, Result, ResultExt, StdResultExt};
use crate::{scmd, dcmd};
use crate::state::{State, Buffer};
use std::io::{self, SeekFrom};
use crate::meta::{Stitch, Trailer, MAGIC};

use crate::de::Deserializer;
use serde::de::DeserializeOwned;

pub struct Serializer {
    state: State,
    first_stitch_pos: u64,
    new_stitches: u64,
    last_trailer_pos: Option<u64>,
}

impl Serializer {
    pub fn new(buf: impl Buffer) -> Result<Serializer> {
        Serializer::from_state(State {
            buf: Box::new(buf),
        })
    }

    pub fn from_state(state: State) -> Result<Serializer> {
        let mut v = Serializer {
            state,
            first_stitch_pos: 0,
            new_stitches: 0,
            last_trailer_pos: None,
        };
        v.reset()?;
        Ok(v)
    }

    pub fn to_state(self) -> State {
        self.state
    }

    pub fn to_de(self) -> Result<Deserializer> {
        Deserializer::from_state(self.to_state())
    }

    pub fn reset(&mut self) -> Result<()> {
        let end_pos = self.state.buf.seek(SeekFrom::End(0)).e()?;
        self.state.buf.seek(SeekFrom::Start(0)).e()?;
        self.first_stitch_pos = end_pos;
        self.new_stitches = 0;
        Ok(())
    }

    fn write(&mut self, v: impl Serialize) -> Result<()> {
        serde_json::to_writer_pretty(&mut self.state.buf, &v).e()?;
        writeln!(&mut self.state.buf).e()?;
        Ok(())
    }

    fn read<T: DeserializeOwned>(&mut self) -> Result<T> {
        let mut de = serde_json::Deserializer::from_reader(&mut self.state.buf);
        let t = T::deserialize(&mut de).e()?;
        Ok(t)
    }

    pub fn finalize(&mut self) -> Result<()> {
        if self.new_stitches == 0 {
            if self.last_trailer_pos.is_some() {
                // No new data written
                return Ok(());
            }
        }
        self.state.buf.seek(SeekFrom::End(0)).e()?;
        let first_stitch = if self.new_stitches != 0 {
            Some(self.first_stitch_pos)
        } else {
            None
        };
        let trailer = Trailer {
            magic: MAGIC,
            first_stitch,
            prev_trailer_pos: self.last_trailer_pos,
        };
        println!("{:?}", trailer);
        self.write(trailer)?;
        Ok(())
    }

    pub fn dump(&mut self) -> Result<()> {
        println!("-- dump --");
        let pos = self.state.buf.seek(SeekFrom::Current(0)).e()?;
        self.state.buf.seek(SeekFrom::Start(0)).e()?;
        let mut stdout = io::stdout();
        io::copy(&mut self.state.buf, &mut stdout).e()?;
        println!("-- dump --");
        self.state.buf.seek(SeekFrom::Start(pos)).e()?;
        Ok(())
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
        let old_pos = self.state.buf.seek(SeekFrom::Current(0)).e()?;
        let newcmd = scmd::SerializeBool { v };
        println!("newcmd: {:?}", newcmd);
        let oldcmd = self.read::<dcmd::SerializeBool>();
        println!("oldcmd: {:?}", oldcmd);
        if oldcmd.cmd_eof() {
            self.write(newcmd)?;
        } else {
            let oldcmd = oldcmd?;
            if oldcmd != newcmd {
                // Save the stream position
                let bookmark_pos = self.state.buf.seek(SeekFrom::Current(0)).e()?;
                // Seek to the end
                let stitch_pos = self.state.buf.seek(SeekFrom::End(0)).e()?;
                // Write a placeholder stitch
                let new_pos = 0;
                let next_stitch_pos = 0;
                let tmp_stitch = Stitch { old_pos, new_pos, next_stitch_pos };
                let tmp_stitch = tmp_stitch.encode();
                self.write(tmp_stitch)?;
                // Write the command
                let new_pos = self.state.buf.seek(SeekFrom::Current(0)).e()?;
                self.write(newcmd)?;
                let next_stitch_pos = self.state.buf.seek(SeekFrom::Current(0)).e()?;
                // Backup and rewrite the real stitch
                self.state.buf.seek(SeekFrom::Start(stitch_pos)).e()?;
                let stitch = Stitch { old_pos, new_pos, next_stitch_pos };
                println!("stitch: {:?}", stitch);
                let stitch = stitch.encode();
                self.write(stitch)?;
                // Verify the stitch size
                let new_pos_2 = self.state.buf.seek(SeekFrom::Current(0)).e()?;
                assert_eq!(new_pos, new_pos_2);
                // Restore the original stream position
                self.state.buf.seek(SeekFrom::Start(bookmark_pos)).e()?;
                // Validate self.first_stitch_pos
                self.new_stitches += 1;
                if self.new_stitches == 1 {
                    assert_eq!(self.first_stitch_pos, stitch_pos);
                }
            }
        }
        Ok(())
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
        println!("serialize_u8");
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
        let newcmd = scmd::SerializeTuple { len };
        println!("newcmd: {:?}", newcmd);
        let oldcmd = self.read::<dcmd::SerializeTuple>();
        println!("oldcmd: {:?}", oldcmd);
        if oldcmd.cmd_eof() {
            self.write(newcmd)?;
        } else {
            let oldcmd = oldcmd?;
            if oldcmd != newcmd {
                unimplemented!()
            }
        }
        Ok(self)
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
        println!("serialize_struct");
        self.write(scmd::SerializeStruct { name, len })?;
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
        let newcmd = scmd::SerializeTupleElement;
        println!("newcmd: {:?}", newcmd);
        let oldcmd = self.read::<dcmd::SerializeTupleElement>();
        println!("oldcmd: {:?}", oldcmd);
        if oldcmd.cmd_eof() {
            self.write(newcmd)?;
        } else {
            let oldcmd = oldcmd?;
            if oldcmd != newcmd {
                unimplemented!()
            }
        }
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<()> {
        let newcmd = scmd::SerializeTupleEnd;
        println!("newcmd: {:?}", newcmd);
        let oldcmd = self.read::<dcmd::SerializeTupleEnd>();
        println!("oldcmd: {:?}", oldcmd);
        if oldcmd.cmd_eof() {
            self.write(newcmd)?;
        } else {
            let oldcmd = oldcmd?;
            if oldcmd != newcmd {
                unimplemented!()
            }
        }
        Ok(())
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
        self.write(scmd::SerializeStructField { key })?;
        value.serialize(&mut **self)?;
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

