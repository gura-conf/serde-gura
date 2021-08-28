// Copyright 2018 Serde Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::error::{Error, Result};
use gura::{parse, GuraType};
use serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess,
    VariantAccess, Visitor,
};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Deserializer {
    obj: GuraType,
}

impl<'de> Deserializer {
    pub fn from_gura_type(obj: GuraType) -> Self {
        Deserializer { obj }
    }
}

// By convention, the public API of a Serde deserializer is one or more
// `from_xyz` methods such as `from_str`, `from_bytes`, or `from_reader`
// depending on what Rust types the deserializer is able to consume as input.
//
// This basic deserializer supports only `from_str`.
pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let parsed = parse(s).map_err(|_| Error::Syntax)?;
    let mut deserializer = Deserializer::from_gura_type(parsed);
    let result = T::deserialize(&mut deserializer)?;
    Ok(result)
}

// SERDE IS NOT A PARSING LIBRARY. This impl block defines a few basic parsing
// functions from scratch. More complicated formats may wish to use a dedicated
// parsing library to help implement their Serde deserializer.
impl<'de> Deserializer {
    // Parse the JSON identifier `true` or `false`.
    fn parse_bool(&mut self) -> Result<bool> {
        if let GuraType::Bool(boolean) = self.obj {
            Ok(boolean)
        } else {
            Err(Error::ExpectedBoolean)
        }
    }

    // Parse a group of decimal digits as an unsigned integer of type T.
    //
    // This implementation is a bit too lenient, for example `001` is not
    // allowed in JSON. Also the various arithmetic operations can overflow and
    // panic or return bogus data. But it is good enough for example code!
    // fn parse_unsigned<T>(&mut self) -> Result<T>
    fn parse_unsigned(&mut self) -> Result<usize> {
        match &self.obj {
            GuraType::Integer(int_value) => Ok(*int_value as usize),
            GuraType::BigInteger(big_int_value) => Ok(*big_int_value as usize),
            _ => Err(Error::ExpectedInteger),
        }
    }

    // Parse a possible minus sign followed by a group of decimal digits as a
    // signed integer of type T.
    fn parse_signed(&mut self) -> Result<isize> {
        match self.obj {
            GuraType::Integer(int_value) => Ok(int_value),
            _ => Err(Error::ExpectedInteger),
        }
    }

    fn parse_float(&mut self) -> Result<f64> {
        match &self.obj {
            GuraType::Float(float_value) => Ok(*float_value),
            _ => Err(Error::ExpectedFloat),
        }
    }

    fn parse_char(&mut self) -> Result<char> {
        if let GuraType::String(str) = &self.obj {
            if str.len() == 1 {
                Ok(str.chars().next().unwrap())
            } else {
                Err(Error::ExpectedChar)
            }
        } else {
            Err(Error::ExpectedChar)
        }
    }

    // Parse a string until the next '"' character.
    //
    // Makes no attempt to handle escape sequences. What did you expect? This is
    // example code!
    fn parse_string(&mut self) -> Result<String> {
        match &self.obj {
            GuraType::String(str_value) => Ok(str_value.clone()),
            GuraType::Pair(key, _, _) => Ok(key.clone()),
            _ => Err(Error::ExpectedString),
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer {
    type Error = Error;

    // Look at the input data to decide what Serde data model type to
    // deserialize as. Not all data formats are able to support this operation.
    // Formats that support `deserialize_any` are known as self-describing.
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.obj {
            GuraType::Integer(_) => self.deserialize_i64(visitor),
            GuraType::Array(_) => self.deserialize_seq(visitor),
            GuraType::Object(_) => self.deserialize_map(visitor),
            _ => Err(Error::Syntax),
        }
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
        visitor.visit_bool(self.parse_bool()?)
    }

    // The `parse_signed` function is generic over the integer type `T` so here
    // it is invoked with `T=i8`. The next 8 methods are similar.
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.parse_signed()? as i8)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.parse_signed()? as i16)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.parse_signed()? as i32)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.parse_signed()? as i64)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.parse_unsigned()? as u8)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.parse_unsigned()? as u16)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parse_unsigned()? as u32)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parse_unsigned()? as u64)
    }

    // Float parsing is stupidly hard.
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.parse_float()? as f32)
    }

    // Float parsing is stupidly hard.
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.parse_float()?)
    }

    // The `Serializer` implementation on the previous page serialized chars as
    // single-character strings so handle that representation here.
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_char(self.parse_char()?)
    }

    // Refer to the "Understanding deserializer lifetimes" page for information
    // about the three deserialization flavors of strings in Serde.
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.parse_string()?)
        // visitor.visit_borrowed_str(&self.parse_string()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    // The `Serializer` implementation on the previous page serialized byte
    // arrays as JSON arrays of bytes. Handle that representation here.
    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
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
        if let GuraType::Null = self.obj {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    // In Serde, unit means an anonymous value containing no data. Gura does not support that kind of type
    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnitNotSupported)
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
        match self.obj {
            GuraType::Array(_) => {
                let obj = self.obj.clone();
                let value = visitor.visit_seq(CommaSeparated::new(obj))?;
                Ok(value)
            }
            _ => Err(Error::ExpectedArray),
        }
    }

    // Tuples look just like sequences in JSON. Some formats may be able to
    // represent tuples more efficiently.
    //
    // As indicated by the length parameter, the `Deserialize` implementation
    // for a tuple in the Serde data model is required to know the length of the
    // tuple before even looking at the input data.
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    // Tuple structs look just like sequences in JSON.
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    // Much like `deserialize_seq` but calls the visitors `visit_map` method
    // with a `MapAccess` implementation, rather than the visitor's `visit_seq`
    // method with a `SeqAccess` implementation.
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let GuraType::Object(_) = &self.obj {
            let obj_aux = self.obj.clone();
            let value = visitor.visit_map(CommaSeparated::new(obj_aux))?;
            Ok(value)
        } else {
            Err(Error::ExpectedMap)
        }
    }

    // Structs look just like maps in JSON.
    //
    // Notice the `fields` parameter - a "struct" in the Serde data model means
    // that the `Deserialize` implementation is required to know what the fields
    // are before even looking at the input data. Any key-value pairing in which
    // the fields cannot be known ahead of time is probably a map.
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
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
        match &self.obj {
            GuraType::String(str) => visitor.visit_enum((str.clone()).into_deserializer()),
            GuraType::Object(_) => visitor.visit_enum(Enum::new(self.obj.clone())),
            _ => Err(Error::ExpectedEnum),
        }
    }

    // An identifier in Serde is the type that identifies a field of a struct or
    // the variant of an enum. In JSON, struct fields and enum variants are
    // represented as strings. In other formats they may be represented as
    // numeric indices.
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
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
        self.deserialize_any(visitor)
    }
}

// In order to handle commas correctly when deserializing a JSON array or map,
// we need to track whether we are on the first element or past the first
// element.
struct CommaSeparated {
    is_empty: bool,
    vec: VecDeque<(String, GuraType)>,
}

impl CommaSeparated {
    fn new(obj: GuraType) -> Self {
        let it: VecDeque<(String, GuraType)> = match obj {
            GuraType::Object(_) => obj
                .iter()
                .unwrap()
                .map(|(key, elem)| (key.clone(), elem.clone()))
                .collect(),
            GuraType::Array(ar) => ar
                .iter()
                .map(|elem| (String::new(), elem.clone()))
                .collect(),
            _ => VecDeque::new(),
        };

        let is_empty = it.is_empty();
        CommaSeparated { vec: it, is_empty }
    }

    fn peek_next_elem(&mut self) -> Result<(String, GuraType)> {
        if let Some(elem) = self.vec.front() {
            Ok(elem.clone())
        } else {
            Err(Error::ExpectedMap)
        }
    }

    fn get_next_elem(&mut self) -> Result<(String, GuraType)> {
        if let Some(elem) = self.vec.pop_front() {
            Ok(elem)
        } else {
            Err(Error::ExpectedMap)
        }
    }
}

// `SeqAccess` is provided to the `Visitor` to give it the ability to iterate
// through elements of the sequence.
impl<'de, 'a> SeqAccess<'de> for CommaSeparated {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if let Ok((_, elem)) = self.get_next_elem() {
            println!("En next_element_seed envia -> {:?}", elem);
            let mut de = Deserializer::from_gura_type(elem);
            seed.deserialize(&mut de).map(Some)
        } else {
            Ok(None)
        }
    }
}

// `MapAccess` is provided to the `Visitor` to give it the ability to iterate
// through entries of the map.
impl<'de, 'a> MapAccess<'de> for CommaSeparated {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        // If it's an empty objects thats represented by 'empty' keyword
        if self.is_empty {
            return Ok(None);
        }

        if let Ok((key, elem)) = self.peek_next_elem() {
            let mut de =
                Deserializer::from_gura_type(GuraType::Pair(key, Box::new(elem), 0));
            seed.deserialize(&mut de).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        let (_, elem) = self.get_next_elem()?;
        let mut de = Deserializer::from_gura_type(elem);
        seed.deserialize(&mut de)
    }
}

#[derive(Debug)]
struct Enum {
    vec: VecDeque<(String, GuraType)>,
}

impl<'a, 'de> Enum {
    fn new(obj: GuraType) -> Self {
        let it: VecDeque<(String, GuraType)> = match obj {
            GuraType::Object(_) => obj
                .iter()
                .unwrap()
                .map(|(key, elem)| (key.clone(), elem.clone()))
                .collect(),
            GuraType::Array(ar) => ar
                .iter()
                .map(|elem| (String::new(), elem.clone()))
                .collect(),
            _ => VecDeque::new(),
        };
        Enum { vec: it }
    }

    fn peek_next_elem(&self) -> Result<(String, GuraType)> {
        if let Some(elem) = self.vec.front() {
            Ok(elem.clone())
        } else {
            Err(Error::ExpectedMap)
        }
    }
}

// `EnumAccess` is provided to the `Visitor` to give it the ability to determine
// which variant of the enum is supposed to be deserialized.
//
// Note that all enum deserialization methods in Serde refer exclusively to the
// "externally tagged" enum representation.
impl<'de, 'a> EnumAccess<'de> for Enum {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        // The `deserialize_enum` method parsed a `{` character so we are
        // currently inside of a map. The seed will be deserializing itself from
        // the key of the map.
        println!("Entra a variant_seed -> {:?}", self);
        let (key, elem) = self.peek_next_elem()?;
        let mut de = Deserializer::from_gura_type(GuraType::Pair(key, Box::new(elem), 0));
        let val = seed.deserialize(&mut de)?;
        Ok((val, self))
    }
}

// `VariantAccess` is provided to the `Visitor` to give it the ability to see
// the content of the single variant that it decided to deserialize.
impl<'de, 'a> VariantAccess<'de> for Enum {
    type Error = Error;

    // If the `Visitor` expected this variant to be a unit variant, the input
    // should have been the plain string case handled in `deserialize_enum`.
    fn unit_variant(self) -> Result<()> {
        Err(Error::ExpectedUnitVariant)
    }

    // Newtype variants are represented in JSON as `{ NAME: VALUE }` so
    // deserialize the value here.
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        let (_, elem) = self.peek_next_elem()?;
        let mut de = Deserializer::from_gura_type(elem);
        seed.deserialize(&mut de)
    }

    // Tuple variants are represented in JSON as `{ NAME: [DATA...] }` so
    // deserialize the sequence of data here.
    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (_, dequeue_as_vec) = self.peek_next_elem()?;
        let de: &mut Deserializer = &mut Deserializer::from_gura_type(dequeue_as_vec);
        de::Deserializer::deserialize_seq(de, visitor)
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }` so
    // deserialize the inner map here.
    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (_, dequeue_as_vec) = self.peek_next_elem()?;
        let de: &mut Deserializer = &mut Deserializer::from_gura_type(dequeue_as_vec);
        de::Deserializer::deserialize_map(de, visitor)
    }
}
