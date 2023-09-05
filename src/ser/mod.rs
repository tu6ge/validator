use std::collections::BTreeMap;

use serde::ser;

use crate::register::{FieldName, FieldNames};

#[cfg(test)]
mod test;

#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum Value {
    UInt8(u8),
    Int8(i8),
    UInt16(u16),
    Int16(i16),
    UInt32(u32),
    Int32(i32),
    UInt64(u64),
    Int64(i64),
    // Float32(f32),
    // Float64(f64),
    String(String),
    Unit,
    // Boolean(bool),
    // Char(char),
    Option(Vec<Value>),
    Array(Vec<Value>),
    Tuple(Vec<Value>),
    TupleStruct(Vec<Value>),
    NewtypeStruct(Vec<Value>),

    Enum(&'static str, Vec<Value>),
    EnumUnit(&'static str),
    TupleVariant(&'static str, Vec<Value>),

    Map(BTreeMap<Value, Value>),

    StructKey(String),
    /// the BtreeMap key only be StructKey(_)
    Struct(BTreeMap<Value, Value>),

    StructVariantKey(String),
    /// the BtreeMap key only be StructVariantKey(_)
    StructVariant(&'static str, BTreeMap<Value, Value>),
}

pub struct ValueMap {
    value: Value,
    index: FieldNames,
}

impl ValueMap {
    pub(crate) fn new(value: Value) -> Self {
        Self {
            value,
            index: FieldNames::new(),
        }
    }
    pub fn index(&mut self, index: FieldNames) {
        self.index = index;
    }
    pub fn current(&self) -> Option<&Value> {
        self.value.get_with_names(&self.index)
    }
    pub fn current_mut(&mut self) -> Option<&mut Value> {
        self.value.get_with_names_mut(&self.index)
    }
    pub fn get(&self, key: &FieldName) -> Option<&Value> {
        self.value.get_with_name(key)
    }
    pub(crate) fn value(self) -> Value {
        self.value
    }
}

pub fn to_value<T>(value: T) -> Result<Value, MyErr>
where
    T: ser::Serialize,
{
    value.serialize(Serializer)
}

impl Value {
    pub fn get_with_name(&self, name: &FieldName) -> Option<&Value> {
        match (name, self) {
            (FieldName::Array(i), Value::Array(vec)) => vec.get(*i),
            (FieldName::Tuple(i), Value::Tuple(vec))
            | (FieldName::Tuple(i), Value::TupleStruct(vec))
            | (FieldName::Tuple(i), Value::NewtypeStruct(vec))
            | (FieldName::Tuple(i), Value::Enum(_, vec))
            | (FieldName::Tuple(i), Value::TupleVariant(_, vec)) => vec.get(*i as usize),
            (FieldName::Literal(str), Value::Struct(btree)) => {
                btree.get(&Value::StructKey(str.to_string()))
            }
            (FieldName::StructVariant(str), Value::StructVariant(_, btree)) => {
                btree.get(&Value::StructVariantKey(str.to_string()))
            }
            _ => None,
        }
    }
    pub fn get_with_names(&self, names: &FieldNames) -> Option<&Value> {
        let mut value = Some(self);
        for name in names.iter() {
            value = match value {
                Some(v) => v.get_with_name(name),
                None => return None,
            }
        }
        value
    }
    pub fn get_with_name_mut(&mut self, name: &FieldName) -> Option<&mut Value> {
        match (name, self) {
            (FieldName::Array(i), Value::Array(vec)) => vec.get_mut(*i),
            (FieldName::Tuple(i), Value::Tuple(vec))
            | (FieldName::Tuple(i), Value::TupleStruct(vec))
            | (FieldName::Tuple(i), Value::NewtypeStruct(vec))
            | (FieldName::Tuple(i), Value::Enum(_, vec))
            | (FieldName::Tuple(i), Value::TupleVariant(_, vec)) => vec.get_mut(*i as usize),
            (FieldName::Literal(str), Value::Struct(btree)) => {
                btree.get_mut(&Value::StructKey(str.to_string()))
            }
            (FieldName::StructVariant(str), Value::StructVariant(_, btree)) => {
                btree.get_mut(&Value::StructVariantKey(str.to_string()))
            }
            _ => None,
        }
    }
    pub fn get_with_names_mut(&mut self, names: &FieldNames) -> Option<&mut Value> {
        let mut value = Some(self);
        for name in names.iter() {
            value = match value {
                Some(v) => v.get_with_name_mut(name),
                None => return None,
            }
        }
        value
    }

    pub(crate) fn get(&self, key: &str) -> Option<&Value> {
        if let Self::Struct(map) = self {
            map.get(&Value::StructKey(key.to_string()))
        } else {
            None
        }
    }
    pub(crate) fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        if let Self::Struct(map) = self {
            map.get_mut(&Value::StructKey(key.to_string()))
        } else {
            None
        }
    }
    pub(crate) fn get_clone(&self, key: &str) -> Option<Value> {
        self.get(key).cloned()
    }
    pub fn is_leaf(&self) -> bool {
        match self {
            Self::UInt8(_)
            | Self::UInt16(_)
            | Self::UInt32(_)
            | Self::UInt64(_)
            | Self::Int8(_)
            | Self::Int16(_)
            | Self::Int32(_)
            | Self::Int64(_) => true,
            Self::Unit => true,
            Self::String(_) => true,
            _ => false,
        }
    }
}

pub(crate) struct Serializer;

#[derive(Debug)]
pub struct MyErr;

impl serde::ser::Error for MyErr {
    fn custom<T>(msg: T) -> Self {
        todo!()
    }
}

impl std::error::Error for MyErr {}
impl std::fmt::Display for MyErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "abc".fmt(f)
    }
}

impl serde::ser::Serializer for Serializer {
    type Ok = Value;

    type Error = MyErr;

    type SerializeSeq = SerializeSeq;

    type SerializeTuple = SerializeTuple;

    type SerializeTupleStruct = SerializeTupleStruct;

    type SerializeTupleVariant = SerializeTupleVariant;

    type SerializeMap = SerializeMap;

    type SerializeStruct = SerializeStruct;

    type SerializeStructVariant = SerializeStructVariant;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(v.to_owned()))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Ok(Value::Option(vec![value.serialize(Serializer)?]))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Unit)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Value::EnumUnit(variant))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Ok(Value::NewtypeStruct(vec![value.serialize(self)?]))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Ok(Value::Enum(variant, vec![value.serialize(self)?]))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        match len {
            Some(len) => Ok(SerializeSeq::with_capacity(len)),
            None => Ok(SerializeSeq::new()),
        }
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(SerializeTuple::with_capacity(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(SerializeTupleStruct::with_capacity(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(SerializeTupleVariant::with_capacity(variant, len))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(SerializeMap::new())
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(SerializeStruct(BTreeMap::default()))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(SerializeStructVariant::new(variant))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Int8(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Int16(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Int32(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Int64(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::UInt8(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::UInt16(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::UInt32(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::UInt64(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Option(vec![]))
    }
}

#[derive(Default)]
pub(crate) struct SerializeSeq(Vec<Value>);

impl SerializeSeq {
    fn new() -> Self {
        Self::default()
    }
    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
}

impl serde::ser::SerializeSeq for SerializeSeq {
    type Error = MyErr;
    type Ok = Value;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.0.push(value.serialize(Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Array(self.0))
    }
}

#[derive(Default)]
pub struct SerializeTuple(Vec<Value>);

impl SerializeTuple {
    fn new() -> Self {
        Self::default()
    }
    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
}

impl ser::SerializeTuple for SerializeTuple {
    type Error = MyErr;
    type Ok = Value;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.0.push(value.serialize(Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Tuple(self.0))
    }
}

#[derive(Default)]
pub struct SerializeTupleStruct(Vec<Value>);

impl SerializeTupleStruct {
    fn new() -> Self {
        Self::default()
    }
    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
}

impl ser::SerializeTupleStruct for SerializeTupleStruct {
    type Error = MyErr;
    type Ok = Value;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.0.push(value.serialize(Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::TupleStruct(self.0))
    }
}
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SerializeTupleVariant {
    variant: &'static str,
    map: Vec<Value>,
}

impl SerializeTupleVariant {
    fn new(variant: &'static str) -> Self {
        Self {
            variant,
            map: Vec::new(),
        }
    }
    fn with_capacity(variant: &'static str, len: usize) -> Self {
        Self {
            variant,
            map: Vec::with_capacity(len),
        }
    }
}
impl ser::SerializeTupleVariant for SerializeTupleVariant {
    type Error = MyErr;
    type Ok = Value;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.map.push(value.serialize(Serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::TupleVariant(self.variant, self.map))
    }
}

pub(crate) struct SerializeMap {
    map: BTreeMap<Value, Value>,
    next_key: Option<Value>,
}

impl SerializeMap {
    fn new() -> Self {
        SerializeMap {
            map: BTreeMap::new(),
            next_key: None,
        }
    }
}

impl ser::SerializeMap for SerializeMap {
    type Error = MyErr;
    type Ok = Value;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.next_key = Some(key.serialize(Serializer)?);

        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        let key = self.next_key.take().unwrap();
        self.map.insert(key, value.serialize(Serializer)?);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Map(self.map))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SerializeStruct(BTreeMap<Value, Value>);
impl ser::SerializeStruct for SerializeStruct {
    type Error = MyErr;
    type Ok = Value;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.0.insert(
            Value::StructKey(key.to_string()),
            value.serialize(Serializer)?,
        );

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Struct(self.0))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SerializeStructVariant {
    variant: &'static str,
    map: BTreeMap<Value, Value>,
}

impl SerializeStructVariant {
    fn new(variant: &'static str) -> Self {
        Self {
            variant,
            map: BTreeMap::new(),
        }
    }
}
impl ser::SerializeStructVariant for SerializeStructVariant {
    type Error = MyErr;
    type Ok = Value;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.map.insert(
            Value::StructVariantKey(key.to_string()),
            value.serialize(Serializer)?,
        );
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::StructVariant(self.variant, self.map))
    }
}
