use serde::{
    ser::{
        self, Serialize, SerializeMap, SerializeSeq, SerializeStruct,
        SerializeStructVariant, SerializeTuple, SerializeTupleStruct,
        SerializeTupleVariant,
    },
    Serializer,
};

use super::Value;
use crate::map::Map;

pub struct ValueSerializer;

/// Represents a generic serialization implementation for `Value`.
impl Serialize for Value {
    fn serialize<S: Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match self {
            Self::Null => serializer.serialize_none(),
            Self::Boolean(b) => serializer.serialize_bool(*b),
            Self::Integer(i) => serializer.serialize_i64(*i),
            Self::Float(f) => serializer.serialize_f64(*f),
            Self::String(s) => serializer.serialize_str(s),
            Self::Array(arr) => {
                let mut seq = serializer.serialize_seq(Some(arr.len()))?;
                for value in arr {
                    seq.serialize_element(value)?;
                }
                seq.end()
            }
            Self::Table(table) => {
                let mut map = serializer.serialize_map(Some(table.len()))?;
                for (key, value) in table {
                    map.serialize_entry(key, value)?;
                }
                map.end()
            }
        }
    }
}

/// A custom serializer that transforms various types into `Value`.
impl Serializer for ValueSerializer {
    type Ok = Value;
    type Error = crate::errors::SerializeError;

    type SerializeSeq = SeqSerializer;
    type SerializeTuple = TupleSerializer;
    type SerializeTupleStruct = TupleStructSerializer;
    type SerializeTupleVariant = TupleVariantSerializer;
    type SerializeMap = MapSerializer;
    type SerializeStruct = StructSerializer;
    type SerializeStructVariant = StructVariantSerializer;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Boolean(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Integer(i64::from(v)))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Integer(i64::from(v)))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Integer(i64::from(v)))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Integer(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Integer(i64::from(v)))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Integer(i64::from(v)))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Integer(i64::from(v)))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        if i64::try_from(v).is_ok() {
            Ok(Value::Integer(v as i64))
        } else {
            Err(serde::ser::Error::custom("u64 value convert failed"))
        }
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Float(f64::from(v)))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Float(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(v.to_string()))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(String::from_utf8_lossy(v).into_owned()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Null)
    }

    fn serialize_some<T: ?Sized + Serialize>(
        self,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Null)
    }

    fn serialize_unit_struct(
        self,
        _name: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unit type not supported"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unit type not supported"))
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_seq(
        self,
        _len: Option<usize>,
    ) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SeqSerializer::new())
    }

    fn serialize_tuple(
        self,
        _len: usize,
    ) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(TupleSerializer::new())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(TupleStructSerializer::new())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(TupleVariantSerializer::new())
    }

    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer::new())
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(StructSerializer::new())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(StructVariantSerializer::new())
    }
}

// Implementations for each serializer
/// Serializer for sequences, storing elements as `Value::Array`.
pub struct SeqSerializer {
    elements: Vec<Value>,
}

impl SeqSerializer {
    /// Constructs a new `SeqSerializer`.
    const fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

/// Serializer for tuples, currently not supported.
pub struct TupleSerializer;
impl TupleSerializer {
    /// Constructs a new `TupleSerializer`.
    const fn new() -> Self {
        Self {}
    }
}
/// Serializer for tuple structs, currently not supported.
pub struct TupleStructSerializer;
impl TupleStructSerializer {
    /// Constructs a new `TupleStructSerializer`.
    const fn new() -> Self {
        Self {}
    }
}
/// Serializer for tuple variants, currently not supported.
pub struct TupleVariantSerializer;
impl TupleVariantSerializer {
    /// Constructs a new `TupleVariantSerializer`.
    const fn new() -> Self {
        Self {}
    }
}
/// Serializer for maps, converting key-value pairs into `Value::Table`.
pub struct MapSerializer {
    map: Map<String, Value>,
    current_key: Option<String>,
}

impl MapSerializer {
    /// Constructs a new `MapSerializer`.
    fn new() -> Self {
        Self {
            map: Map::new(),
            current_key: None,
        }
    }
}

/// Serializer for structs, storing fields as `Value::Table` or special cases.
pub struct StructSerializer {
    fields: Value,
}

impl StructSerializer {
    /// Constructs a new `StructSerializer`.
    const fn new() -> Self {
        Self {
            fields: Value::Null,
        }
    }
}
/// Serializer for struct variants, currently not supported.
pub struct StructVariantSerializer;

impl StructVariantSerializer {
    /// Constructs a new `StructVariantSerializer`.
    const fn new() -> Self {
        Self {}
    }
}

impl SerializeSeq for SeqSerializer {
    type Ok = Value;
    type Error = crate::errors::SerializeError;

    fn serialize_element<T: ?Sized + Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error> {
        let element = value.serialize(ValueSerializer)?;
        self.elements.push(element);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Array(self.elements))
    }
}

impl SerializeTuple for TupleSerializer {
    type Ok = Value;
    type Error = crate::errors::SerializeError;

    fn serialize_element<T: ?Sized + Serialize>(
        &mut self,
        _value: &T,
    ) -> Result<(), Self::Error> {
        Err(ser::Error::custom("tuple type not supported"))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("tuple type not supported"))
    }
}

impl SerializeTupleStruct for TupleStructSerializer {
    type Ok = Value;
    type Error = crate::errors::SerializeError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        _value: &T,
    ) -> Result<(), Self::Error> {
        Err(ser::Error::custom("tuple struct type not supported"))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("tuple struct type not supported"))
    }
}

impl SerializeTupleVariant for TupleVariantSerializer {
    type Ok = Value;
    type Error = crate::errors::SerializeError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        _value: &T,
    ) -> Result<(), Self::Error> {
        Err(ser::Error::custom("tuple variant type not supported"))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("tuple variant type not supported"))
    }
}

impl SerializeMap for MapSerializer {
    type Ok = Value;
    type Error = crate::errors::SerializeError;

    fn serialize_key<K: ?Sized + Serialize>(
        &mut self,
        key: &K,
    ) -> Result<(), Self::Error> {
        let serialized_key = key.serialize(ValueSerializer)?;
        if let Value::String(key_str) = serialized_key {
            self.current_key = Some(key_str);
            Ok(())
        } else {
            Err(ser::Error::custom("Only string keys are supported"))
        }
    }

    fn serialize_value<V: ?Sized + Serialize>(
        &mut self,
        value: &V,
    ) -> Result<(), Self::Error> {
        if let Some(ref key) = self.current_key {
            let serialized_value = value.serialize(ValueSerializer)?;
            self.map.insert(key.clone(), serialized_value);
            self.current_key = None;
            Ok(())
        } else {
            Err(ser::Error::custom("No current key"))
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Table(self.map))
    }
}

impl SerializeStruct for StructSerializer {
    type Ok = Value;
    type Error = crate::errors::SerializeError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let serialized_value = value.serialize(ValueSerializer)?;

        // Handle special case for TOML datetime serialization
        // If the key is "$__toml_private_datetime", it's a special case for
        // TOML where the entire struct should be replaced with the
        // serialized datetime value. This is only relevant when the
        // "toml" feature is enabled.
        #[cfg(feature = "toml")]
        if key == "$__toml_private_datetime" {
            self.fields = serialized_value;
            return Ok(());
        }

        match self.fields {
            Value::Null => {
                let mut map = Map::new();
                map.insert(key.to_string(), serialized_value);
                self.fields = Value::Table(map);
            }
            Value::Table(ref mut t) => {
                t.insert(key.to_string(), serialized_value);
            }
            _ => {
                return Err(ser::Error::custom(
                    "something not expect to happen",
                ));
            }
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.fields)
    }
}

impl SerializeStructVariant for StructVariantSerializer {
    type Ok = Value;
    type Error = crate::errors::SerializeError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        Err(ser::Error::custom("struct variant type not supported"))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("struct variant type not supported"))
    }
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[test]
    fn test_struct() {
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
        struct Test {
            int: u32,
            seq: Vec<String>,
        }

        let test = Test {
            int: 1,
            seq: vec!["a".to_string(), "b".to_string()],
        };
        let config = Value::try_serialize(&test).unwrap();
        let actual: Test = config.try_deserialize().unwrap();
        assert_eq!(test, actual);
    }

    #[test]
    fn test_nest() {
        let val = serde_json::json! { {
            "top": {
                "num": 1,
                "array": [2],
                "nested": [[3,4]],
                "deep": [{
                    "yes": true,
                }],
                "mixed": [
                    { "boolish": false, },
                    42,
                    ["hi"],
                    { "inner": 66 },
                    23,
                ],
            }
        } };
        let config = Value::try_serialize(&val).unwrap();
        let output: serde_json::Value = config.try_deserialize().unwrap();
        assert_eq!(val, output);
    }
}
