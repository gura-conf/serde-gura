use super::error::{Error, Result};
use gura::{dump, GuraType};
use indexmap::IndexMap;
use serde::ser;

pub struct Serializer;

impl ser::Serializer for Serializer {
    type Ok = GuraType;
    type Error = Error;

    type SerializeSeq = SerializeArray;
    type SerializeTuple = SerializeArray;
    type SerializeTupleStruct = SerializeArray;
    type SerializeTupleVariant = SerializeTupleVariant;
    type SerializeMap = SerializeMap;
    type SerializeStruct = SerializeStruct;
    type SerializeStructVariant = SerializeStructVariant;

    fn serialize_bool(self, v: bool) -> Result<GuraType> {
        Ok(GuraType::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<GuraType> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i16(self, v: i16) -> Result<GuraType> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(self, v: i32) -> Result<GuraType> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(self, v: i64) -> Result<GuraType> {
        Ok(GuraType::Integer(v as isize))
    }

    fn serialize_u8(self, v: u8) -> Result<GuraType> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u16(self, v: u16) -> Result<GuraType> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u32(self, v: u32) -> Result<GuraType> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u64(self, v: u64) -> Result<GuraType> {
        Ok(GuraType::Integer(v as isize))
    }

    fn serialize_f32(self, v: f32) -> Result<GuraType> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(self, v: f64) -> Result<GuraType> {
        Ok(GuraType::Float(v))
    }

    fn serialize_char(self, value: char) -> Result<GuraType> {
        self.serialize_str(&value.to_string())
    }

    fn serialize_str(self, value: &str) -> Result<GuraType> {
        Ok(GuraType::String(value.to_string()))
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<GuraType> {
        let vec = value
            .iter()
            .map(|&b| GuraType::Integer(b as isize))
            .collect();
        Ok(GuraType::Array(vec))
    }

    fn serialize_unit(self) -> Result<GuraType> {
        Ok(GuraType::Null)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<GuraType> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &str,
        _variant_index: u32,
        variant: &str,
    ) -> Result<GuraType> {
        Ok(GuraType::String(variant.to_owned()))
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<GuraType>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &str,
        _variant_index: u32,
        variant: &str,
        value: &T,
    ) -> Result<GuraType>
    where
        T: ser::Serialize,
    {
        Ok(singleton_hash(variant.to_string(), to_gura_type(value)?))
    }

    fn serialize_none(self) -> Result<GuraType> {
        self.serialize_unit()
    }

    fn serialize_some<V: ?Sized>(self, value: &V) -> Result<GuraType>
    where
        V: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<SerializeArray> {
        let array = match len {
            None => Vec::new(),
            Some(len) => Vec::with_capacity(len),
        };
        Ok(SerializeArray { array })
    }

    fn serialize_tuple(self, len: usize) -> Result<SerializeArray> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<SerializeArray> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _enum: &'static str,
        _idx: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<SerializeTupleVariant> {
        Ok(SerializeTupleVariant {
            name: variant,
            array: Vec::with_capacity(len),
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<SerializeMap> {
        Ok(SerializeMap {
            hash: IndexMap::new(),
            next_key: None,
        })
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<SerializeStruct> {
        Ok(SerializeStruct {
            hash: IndexMap::new(),
        })
    }

    fn serialize_struct_variant(
        self,
        _enum: &'static str,
        _idx: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<SerializeStructVariant> {
        Ok(SerializeStructVariant {
            name: variant,
            hash: IndexMap::new(),
        })
    }
}

#[doc(hidden)]
pub struct SerializeArray {
    array: Vec<GuraType>,
}

#[doc(hidden)]
pub struct SerializeTupleVariant {
    name: &'static str,
    array: Vec<GuraType>,
}

#[doc(hidden)]
pub struct SerializeMap {
    hash: IndexMap<String, GuraType>, // Must to be a hash
    next_key: Option<String>,
}

#[doc(hidden)]
pub struct SerializeStruct {
    hash: IndexMap<String, GuraType>, // Must to be a hash
}

#[doc(hidden)]
pub struct SerializeStructVariant {
    name: &'static str,
    hash: IndexMap<String, GuraType>, // Must to be a hash
}

impl ser::SerializeSeq for SerializeArray {
    type Ok = GuraType;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, elem: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        self.array.push(to_gura_type(elem)?);
        Ok(())
    }

    fn end(self) -> Result<GuraType> {
        Ok(GuraType::Array(self.array))
    }
}

impl ser::SerializeTuple for SerializeArray {
    type Ok = GuraType;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, elem: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, elem)
    }

    fn end(self) -> Result<GuraType> {
        ser::SerializeSeq::end(self)
    }
}

impl ser::SerializeTupleStruct for SerializeArray {
    type Ok = GuraType;
    type Error = Error;

    fn serialize_field<V: ?Sized>(&mut self, value: &V) -> Result<()>
    where
        V: ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<GuraType> {
        ser::SerializeSeq::end(self)
    }
}

impl ser::SerializeTupleVariant for SerializeTupleVariant {
    type Ok = GuraType;
    type Error = Error;

    fn serialize_field<V: ?Sized>(&mut self, v: &V) -> Result<()>
    where
        V: ser::Serialize,
    {
        self.array.push(to_gura_type(v)?);
        Ok(())
    }

    fn end(self) -> Result<GuraType> {
        Ok(singleton_hash(
            self.name.to_string(),
            GuraType::Array(self.array),
        ))
    }
}

impl ser::SerializeMap for SerializeMap {
    type Ok = GuraType;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        self.next_key = Some(to_gura_type(key)?.to_string());
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        match self.next_key.take() {
            Some(key) => self.hash.insert(key, to_gura_type(value)?),
            None => panic!("Serialize_value called before serialize_key"),
        };
        Ok(())
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(&mut self, key: &K, value: &V) -> Result<()>
    where
        K: ser::Serialize,
        V: ser::Serialize,
    {
        let res = to_gura_type(key)?.to_string();
        self.hash.insert(res, to_gura_type(value)?);
        Ok(())
    }

    fn end(self) -> Result<GuraType> {
        Ok(GuraType::Object(self.hash))
    }
}

impl ser::SerializeStruct for SerializeStruct {
    type Ok = GuraType;
    type Error = Error;

    fn serialize_field<V: ?Sized>(&mut self, key: &'static str, value: &V) -> Result<()>
    where
        V: ser::Serialize,
    {
        self.hash.insert(key.to_string(), to_gura_type(value)?);
        Ok(())
    }

    fn end(self) -> Result<GuraType> {
        Ok(GuraType::Object(self.hash))
    }
}

impl ser::SerializeStructVariant for SerializeStructVariant {
    type Ok = GuraType;
    type Error = Error;

    fn serialize_field<V: ?Sized>(&mut self, field: &'static str, v: &V) -> Result<()>
    where
        V: ser::Serialize,
    {
        self.hash.insert(field.to_string(), to_gura_type(v)?);
        Ok(())
    }

    fn end(self) -> Result<GuraType> {
        Ok(singleton_hash(
            self.name.to_string(),
            GuraType::Object(self.hash),
        ))
    }
}

/// Serialize the given data structure as a String of Gura.
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// return an error.
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: ser::Serialize,
{
    let serializer = Serializer {};
    let result = value.serialize(serializer)?;
    Ok(dump(&result))
}


fn to_gura_type<T>(elem: T) -> Result<GuraType>
where
    T: ser::Serialize,
{
    elem.serialize(Serializer)
}

fn singleton_hash(k: String, v: GuraType) -> GuraType {
    let mut hash = IndexMap::new();
    hash.insert(k, v);
    GuraType::Object(hash)
}
