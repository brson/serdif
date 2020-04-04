#![allow(unused)]

use crate::error::{Error, Result, ResultExt};
use crate::dcmd;
use crate::state::{State, Buffer};
use std::io::SeekFrom;

use std::ops::{AddAssign, MulAssign, Neg};

use serde::Deserialize;
use serde::de::{
    self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess,
    VariantAccess, Visitor, DeserializeOwned
};

use crate::ser::Serializer;

pub struct Deserializer {
    state: State,
}

impl Deserializer {
    pub fn new(buf: impl Buffer) -> Deserializer {
        Deserializer::from_state(State {
            buf: Box::new(buf),
        })
    }

    pub fn from_state(state: State) -> Deserializer {
        Deserializer { state }
    }

    pub fn to_state(self) -> State {
        self.state
    }

    pub fn to_ser(self) -> Serializer {
        Serializer::from_state(self.to_state())
    }

    pub fn reset(&mut self) -> Result<()> {
        self.state.buf.seek(SeekFrom::Start(0)).e()?;
        Ok(())
    }

    fn read<T: DeserializeOwned>(&mut self) -> Result<T> {
        let mut de = serde_json::Deserializer::from_reader(&mut self.state.buf);
        let t = T::deserialize(&mut de).e()?;
        Ok(t)
        //Ok(serde_json::from_reader(&mut self.state.buf).e()?)
    }
}

impl<'a> de::Deserializer<'static> for &'a mut Deserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        panic!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        println!("deserialize_bool");
        let cmd = self.read::<dcmd::SerializeBool>()?;
        visitor.visit_bool(cmd.v)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        visitor.visit_i8(panic!())
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        visitor.visit_i16(panic!())
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        visitor.visit_i32(panic!())
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        visitor.visit_i64(panic!())
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        println!("deserialize_u8");
        let cmd = self.read::<dcmd::SerializeU8>()?;
        visitor.visit_u8(cmd.v)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        visitor.visit_u16(panic!())
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        visitor.visit_u32(panic!())
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        visitor.visit_u64(panic!())
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        visitor.visit_borrowed_str(panic!())
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        panic!()
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        panic!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        panic!()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        panic!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        panic!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        println!("deserialize_tuple");
        println!("len: {}", len);
        let cmd = self.read::<dcmd::SerializeTuple>()?;
        assert_eq!(len, cmd.len);

        struct Access<'a> {
            de: &'a mut Deserializer,
            len: usize,
        }

        impl<'a> de::SeqAccess<'static> for Access<'a> {
            type Error = Error;

            fn next_element_seed<T>(&mut self, seed: T)
                                    -> Result<Option<T::Value>>
            where T: de::DeserializeSeed<'static>,
            {
                println!("next_element_seed(tuple)");
                if self.len > 0 {
                    let cmd = self.de.read::<dcmd::SerializeTupleElement>()?;
                    self.len -= 1;
                    let value =
                        de::DeserializeSeed::deserialize(seed, &mut *self.de)?;
                    Ok(Some(value))
                } else {
                    let cmd = self.de.read::<dcmd::SerializeTupleEnd>()?;
                    Ok(None)
                }
            }

            fn size_hint(&self) -> Option<usize> {
                Some(self.len)
            }
        }

        visitor.visit_seq(Access {
            de: self,
            len,
        })
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        panic!()
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        panic!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        println!("deserialize_struct");
        println!("name: {}", name);
        println!("fields: {:#?}", fields);
        let cmd = self.read::<dcmd::SerializeStruct>()?;
        panic!()
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        panic!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        panic!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'static>,
    {
        panic!()
    }
}

