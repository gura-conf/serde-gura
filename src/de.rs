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

// This deserializer supports only `from_str` for the moment
pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let parsed = parse(s).map_err(|e| Error::Syntax(e.to_string()))?;
    let mut deserializer = Deserializer::from_gura_type(parsed);
    let result = T::deserialize(&mut deserializer)?;
    Ok(result)
}

// Serde is not a parsing library. That's why Gura Rust parser is used in this crate
impl<'de> Deserializer {
    fn parse_bool(&mut self) -> Result<bool> {
        if let GuraType::Bool(boolean) = self.obj {
            Ok(boolean)
        } else {
            Err(Error::ExpectedBoolean)
        }
    }

    fn parse_unsigned(&mut self) -> Result<usize> {
        match &self.obj {
            GuraType::Integer(int_value) => Ok(*int_value as usize),
            GuraType::BigInteger(big_int_value) => Ok(*big_int_value as usize),
            _ => Err(Error::ExpectedInteger),
        }
    }

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

    fn parse_string(&mut self) -> Result<String> {
        match &self.obj {
            GuraType::Pair(key, _, _) => Ok(key.clone()),
            GuraType::String(str_value) => Ok(str_value.clone()),
            _ => Err(Error::ExpectedString),
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer {
    type Error = Error;

    // Look at the input data to decide what Serde data model type to deserialize as
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.obj {
            GuraType::Array(_) => self.deserialize_seq(visitor),
            GuraType::BigInteger(_) => self.deserialize_i128(visitor),
            GuraType::Bool(_) => self.deserialize_bool(visitor),
            GuraType::Float(_) => self.deserialize_f64(visitor),
            GuraType::Integer(_) => self.deserialize_i64(visitor),
            GuraType::Null => self.deserialize_unit(visitor),
            GuraType::Object(_) => self.deserialize_map(visitor),
            GuraType::Pair(..) => self.deserialize_identifier(visitor),
            GuraType::String(_) => self.deserialize_string(visitor),
            _ => Err(Error::InvalidType),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.parse_bool()?)
    }

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

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.parse_float()? as f32)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.parse_float()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_char(self.parse_char()?)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.parse_string()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

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

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnitNotSupported)
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

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

    // Tuples look just like sequences in Gura (arrays)
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    // Tuple structs look just like sequences in Gura.
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
    // the variant of an enum. In Gura, struct fields and enum variants are
    // represented as strings.
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.obj {
            GuraType::Pair(key, _, _) => visitor.visit_string(key.clone()),
            GuraType::String(str) => visitor.visit_string(str.to_string()),
            _ => Err(Error::ExpectedIdentifier),
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

// Struct helper to parse Gura objects and arrays
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
            let mut de = Deserializer::from_gura_type(GuraType::Pair(key, Box::new(elem), 0));
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
// which variant of the enum is supposed to be deserialized
impl<'de, 'a> EnumAccess<'de> for Enum {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        // Generates a Gura pair to check the Serde variant
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

    // Deserializes the variant value here
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        let (_, elem) = self.peek_next_elem()?;
        let mut de = Deserializer::from_gura_type(elem);
        seed.deserialize(&mut de)
    }

    // Deserializes the sequence of data here
    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (_, dequeue_as_vec) = self.peek_next_elem()?;
        let de: &mut Deserializer = &mut Deserializer::from_gura_type(dequeue_as_vec);
        de::Deserializer::deserialize_seq(de, visitor)
    }

    // Deserializes the inner map here
    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let (_, dequeue_as_vec) = self.peek_next_elem()?;
        let de: &mut Deserializer = &mut Deserializer::from_gura_type(dequeue_as_vec);
        de::Deserializer::deserialize_map(de, visitor)
    }
}
