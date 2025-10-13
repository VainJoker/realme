use std::fmt;

use serde::{
    Deserialize,
    de::{
        self,
        Deserializer,
        IntoDeserializer,
        Visitor,
    },
};

use super::{
    Table,
    Value,
};
use crate::Map;

/// Represents a custom deserializer for `Value` type.
impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}

/// A visitor to handle custom deserialization logic for `Value`.
struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid Value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
        Ok(Value::Boolean(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
        Ok(Value::Integer(value))
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Value, E> {
        if i64::try_from(value).is_ok() {
            Ok(Value::Integer(value as i64))
        } else {
            Err(de::Error::custom("u64 value was too large"))
        }
    }

    fn visit_u32<E>(self, value: u32) -> Result<Value, E> {
        Ok(Value::Integer(value.into()))
    }

    fn visit_i32<E>(self, value: i32) -> Result<Value, E> {
        Ok(Value::Integer(value.into()))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
        Ok(Value::Float(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<Value, E> {
        Ok(Value::String(value.into()))
    }

    fn visit_string<E>(self, value: String) -> Result<Value, E> {
        Ok(Value::String(value))
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(deserializer)
    }

    fn visit_seq<V>(self, mut visitor: V) -> Result<Value, V::Error>
    where
        V: de::SeqAccess<'de>,
    {
        let mut vec = Vec::new();
        while let Some(elem) = visitor.next_element()? {
            vec.push(elem);
        }
        Ok(Value::Array(vec))
    }

    fn visit_map<M>(self, mut access: M) -> Result<Value, M::Error>
    where
        M: de::MapAccess<'de>,
    {
        let mut map = Map::new();

        while let Some((key, value)) = access.next_entry()? {
            map.insert(key, value);
        }

        Ok(Value::Table(map))
    }
}

/// Custom deserializer implementation for `Value`.
impl<'de> serde::Deserializer<'de> for Value {
    type Error = crate::errors::DeserializeError;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        #[allow(unreachable_patterns)]
        match self {
            Self::Null => visitor.visit_none(),
            Self::String(s) => visitor.visit_str(&s),
            Self::Integer(i) => visitor.visit_i64(i),
            Self::Boolean(b) => visitor.visit_bool(b),
            Self::Float(f) => visitor.visit_f64(f),
            Self::Array(a) => visitor.visit_seq(SeqDeserializer::new(a)),
            Self::Table(t) => visitor.visit_map(MapDeserializer::new(t)),
            _ => Err(de::Error::custom(format!(
                "unsupported type for any: {}, value: {:?}",
                self.value_type(),
                self
            ))),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let s: String = self
            .try_into()
            .map_err(|e: crate::Error| de::Error::custom(e.to_string()))?;
        visitor.visit_str(&s)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let i: i64 = self
            .try_into()
            .map_err(|e: crate::Error| de::Error::custom(e.to_string()))?;
        visitor.visit_i64(i)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let i: bool = self
            .try_into()
            .map_err(|e: crate::Error| de::Error::custom(e.to_string()))?;
        visitor.visit_bool(i)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let i: i64 = self
            .try_into()
            .map_err(|e: crate::Error| de::Error::custom(e.to_string()))?;
        visitor.visit_i8(i as i8)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let i: i64 = self
            .try_into()
            .map_err(|e: crate::Error| de::Error::custom(e.to_string()))?;
        visitor.visit_i16(i as i16)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let i: i64 = self
            .try_into()
            .map_err(|e: crate::Error| de::Error::custom(e.to_string()))?;
        visitor.visit_i32(i as i32)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let u: u64 = self
            .try_into()
            .map_err(|e: crate::Error| de::Error::custom(e.to_string()))?;
        visitor.visit_u8(u as u8)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let u: u64 = self
            .try_into()
            .map_err(|e: crate::Error| de::Error::custom(e.to_string()))?;
        visitor.visit_u16(u as u16)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let u: u64 = self
            .try_into()
            .map_err(|e: crate::Error| de::Error::custom(e.to_string()))?;
        visitor.visit_u32(u as u32)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let u: u64 = self
            .try_into()
            .map_err(|e: crate::Error| de::Error::custom(e.to_string()))?;
        visitor.visit_u64(u)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let f: f64 = self
            .try_into()
            .map_err(|e: crate::Error| de::Error::custom(e.to_string()))?;
        visitor.visit_f32(f as f32)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let f: f64 = self
            .try_into()
            .map_err(|e: crate::Error| de::Error::custom(e.to_string()))?;
        visitor.visit_f64(f)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let s: String = self
            .try_into()
            .map_err(|e: crate::Error| de::Error::custom(e.to_string()))?;
        visitor.visit_str(&s)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let s: String = self
            .try_into()
            .map_err(|e: crate::Error| de::Error::custom(e.to_string()))?;
        visitor.visit_str(&s)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let s: String = self
            .try_into()
            .map_err(|e: crate::Error| de::Error::custom(e.to_string()))?;
        visitor.visit_str(&s)
    }

    fn deserialize_byte_buf<V>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let s: String = self
            .try_into()
            .map_err(|e: crate::Error| de::Error::custom(e.to_string()))?;
        visitor.visit_str(&s)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self == Self::Null {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let seq = self
            .try_into()
            .map_err(|e: crate::Error| de::Error::custom(e.to_string()))?;
        visitor.visit_seq(SeqDeserializer::new(seq))
    }

    fn deserialize_tuple<V>(
        self,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let seq = self
            .try_into()
            .map_err(|e: crate::Error| de::Error::custom(e.to_string()))?;
        visitor.visit_seq(SeqDeserializer::new(seq))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom(format!(
            "unsupported type for tuple struct: {}, value: {:?}",
            self.value_type(),
            self
        )))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Self::Table(t) => visitor.visit_map(MapDeserializer::new(t)),
            _ => Err(de::Error::custom(format!(
                "expected a table, got {}, value: {:?}",
                self.value_type(),
                self
            ))),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Self::Table(t) => visitor.visit_map(MapDeserializer::new(t)),
            _ => Err(de::Error::custom(format!(
                "expected a table, got {}, value: {:?}",
                self.value_type(),
                self
            ))),
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Self::String(s) => {
                visitor.visit_enum(EnumDeserializer::new(EnumVariant::Unit(s)))
            }
            Self::Table(_) => Err(de::Error::custom(
                "enum with tuple or struct variant is not supported"
                    .to_string(),
            )),
            _ => Err(de::Error::custom(format!(
                "expected a string, got {}, value: {:?}",
                self.value_type(),
                self
            ))),
        }
    }

    fn deserialize_identifier<V>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Self::String(s) => visitor.visit_str(&s),
            _ => Err(de::Error::custom(format!(
                "expected a string, got {}, value: {:?}",
                self.value_type(),
                self
            ))),
        }
    }

    fn deserialize_ignored_any<V>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Self::Null => visitor.visit_none(),
            Self::Boolean(b) => visitor.visit_bool(b),
            Self::Integer(i) => visitor.visit_i64(i),
            Self::Float(f) => visitor.visit_f64(f),
            Self::String(s) => visitor.visit_str(&s),
            Self::Array(a) => visitor.visit_seq(SeqDeserializer::new(a)),
            Self::Table(t) => visitor.visit_map(MapDeserializer::new(t)),
        }
    }
}

/// A helper struct to facilitate map deserialization.
struct MapDeserializer {
    iter:  <Map<String, Value> as IntoIterator>::IntoIter,
    value: Option<(String, Value)>,
}

impl MapDeserializer {
    fn new(map: Map<String, Value>) -> Self {
        Self {
            iter:  map.into_iter(),
            value: None,
        }
    }
}

impl<'de> de::MapAccess<'de> for MapDeserializer {
    type Error = crate::errors::DeserializeError;

    fn next_key_seed<T>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some((key.clone(), value));
                seed.deserialize(Value::String(key)).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        let (_, res) = match self.value.take() {
            Some((key, value)) => (key, seed.deserialize(value)),
            None => return Err(de::Error::custom("value is missing")),
        };
        res.map_err(de::Error::custom)
    }

    fn size_hint(&self) -> Option<usize> {
        match self.iter.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}

/// A helper struct to facilitate sequence deserialization.
struct SeqDeserializer {
    iter: <Vec<Value> as IntoIterator>::IntoIter,
}

impl<'de> de::SeqAccess<'de> for SeqDeserializer {
    type Error = crate::errors::DeserializeError;

    fn next_element_seed<T>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => seed.deserialize(value).map(Some),
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        match self.iter.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}

impl SeqDeserializer {
    fn new(seq: Vec<Value>) -> Self {
        Self {
            iter: seq.into_iter(),
        }
    }
}

/// A helper struct to facilitate map deserialization.
struct EnumDeserializer {
    variant: EnumVariant,
}

enum EnumVariant {
    Unit(String),
    #[allow(dead_code)]
    Tuple(Table),
    #[allow(dead_code)]
    Struct(Table),
}

impl EnumDeserializer {
    const fn new(variant: EnumVariant) -> Self {
        Self { variant }
    }
}

struct EnumVariantDeserializer;

impl<'de> de::VariantAccess<'de> for EnumVariantDeserializer {
    type Error = crate::errors::DeserializeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        Err(de::Error::custom("newtype variant is not supported"))
    }

    fn tuple_variant<V>(
        self,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("tuple variant is not supported"))
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("struct variant is not supported"))
    }
}

impl<'de> de::EnumAccess<'de> for EnumDeserializer {
    type Error = crate::errors::DeserializeError;

    type Variant = EnumVariantDeserializer;

    fn variant_seed<V>(
        self,
        seed: V,
    ) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let EnumVariant::Unit(key) = self.variant else {
            return Err(de::Error::custom("not a unit variant".to_string()));
        };

        let val = seed.deserialize(key.into_deserializer())?;

        let variant = EnumVariantDeserializer;

        Ok((val, variant))
    }
}

/// Unit tests to verify the functionality of the custom deserializer.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::Error;

    #[test]
    fn test_deserialize_string() -> anyhow::Result<()> {
        let value = Value::String("test".to_string());
        let result: String = value.try_deserialize()?;
        assert_eq!(result, "test");
        Ok(())
    }

    #[test]
    fn test_deserialize_integer() -> anyhow::Result<()> {
        let value = Value::Integer(42);
        let result: i64 = value.try_deserialize()?;
        assert_eq!(result, 42);
        Ok(())
    }

    #[test]
    fn test_deserialize_string_to_integer_but_should_failed() {
        let value = Value::String("test".to_string());
        let result: Result<i64, Error> = value.try_deserialize();
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_boolean() -> anyhow::Result<()> {
        let value = Value::Boolean(true);
        let result: bool = value.try_deserialize()?;
        assert!(result);
        Ok(())
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_deserialize_float() -> anyhow::Result<()> {
        let value = Value::Float(0.618);
        let result: f64 = value.try_deserialize()?;
        assert_eq!(result, 0.618_f64);
        Ok(())
    }

    #[test]
    fn test_deserialize_array() -> anyhow::Result<()> {
        let value = Value::Array(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3),
        ]);
        let result: Vec<i64> = value.try_deserialize()?;
        assert_eq!(result, vec![1, 2, 3]);
        Ok(())
    }

    #[test]
    fn test_deserialize_array_of_array() -> anyhow::Result<()> {
        let value = Value::Array(vec![
            Value::Array(vec![Value::Integer(1), Value::Integer(2)]),
            Value::Array(vec![Value::Integer(3), Value::Integer(4)]),
        ]);
        let result: Vec<Vec<i64>> = value.try_deserialize()?;
        assert_eq!(result, vec![vec![1, 2], vec![3, 4]]);
        Ok(())
    }

    #[test]
    fn test_deserialize_table() -> anyhow::Result<()> {
        let mut map = Map::new();
        map.insert("key".to_string(), Value::String("value".to_string()));
        let value = Value::Table(map);
        let result: std::collections::HashMap<String, String> =
            value.try_deserialize()?;
        assert_eq!(result.get("key"), Some(&"value".to_string()));
        Ok(())
    }

    #[test]
    fn test_deserialize_nested_structure() -> anyhow::Result<()> {
        let mut inner_map = Map::new();
        inner_map.insert("inner_key".to_string(), Value::Integer(42));
        let mut outer_map = Map::new();
        outer_map.insert("outer_key".to_string(), Value::Table(inner_map));
        let value = Value::Table(outer_map);
        #[allow(clippy::items_after_statements)]
        #[derive(Debug, Deserialize, PartialEq)]
        struct InnerStruct {
            inner_key: i64,
        }
        #[allow(clippy::items_after_statements)]
        #[derive(Debug, Deserialize, PartialEq)]
        struct NestedStruct {
            outer_key: InnerStruct,
        }

        let result: NestedStruct = value.try_deserialize()?;
        assert_eq!(result, NestedStruct {
            outer_key: InnerStruct { inner_key: 42 },
        });
        Ok(())
    }

    #[test]
    fn test_deserialize_u64() -> anyhow::Result<()> {
        let value = Value::Integer(42);
        let result: u64 = value.try_deserialize()?;
        assert_eq!(result, 42);
        Ok(())
    }

    #[test]
    fn test_deserialize_option_some() -> anyhow::Result<()> {
        let value = Value::String("test".to_string());
        let result: Option<String> = value.try_deserialize()?;
        assert_eq!(result, Some("test".to_string()));
        Ok(())
    }

    #[test]
    fn test_deserialize_option_none() -> anyhow::Result<()> {
        let value = Value::Null;
        let result: Option<String> = value.try_deserialize()?;
        assert_eq!(result, None);
        Ok(())
    }

    #[test]
    fn test_deserialize_char() -> anyhow::Result<()> {
        let value = Value::String("a".to_string());
        let result: char = value.try_deserialize()?;
        assert_eq!(result, 'a');
        Ok(())
    }

    #[test]
    fn test_deserialize_tuple() -> anyhow::Result<()> {
        let value = Value::Array(vec![Value::Integer(1), Value::Integer(2)]);
        let result: (i64, i64) = value.try_deserialize()?;
        assert_eq!(result, (1, 2));
        Ok(())
    }

    #[cfg(test)]
    mod enum_deserialization_tests {
        use super::*;

        #[derive(Debug, Deserialize, PartialEq)]
        #[serde(rename_all = "snake_case")]
        enum TestEnum {
            Variant1,
            Variant2(i64),
            Variant3 { field: String },
        }

        #[test]
        fn test_deserialize_unit_variant() -> anyhow::Result<()> {
            let value = Value::String("variant1".to_string());
            let result: TestEnum = value.try_deserialize()?;
            assert_eq!(result, TestEnum::Variant1);
            Ok(())
        }

        #[test]
        fn test_deserialize_newtype_variant_fails() {
            let value = Value::String("variant2".to_string());
            let result: Result<TestEnum, Error> = value.try_deserialize();
            assert!(result.is_err());
        }

        #[test]
        fn test_deserialize_struct_variant_fails() {
            let mut map = Map::new();
            map.insert("variant3".to_string(), Value::Table(Map::new()));
            let value = Value::Table(map);
            let result: Result<TestEnum, Error> = value.try_deserialize();
            assert!(result.is_err());
        }

        #[test]
        fn test_deserialize_non_existent_variant() {
            let value = Value::String("variant4".to_string());
            let result: Result<TestEnum, Error> = value.try_deserialize();
            assert!(result.is_err());
        }

        #[test]
        fn test_deserialize_enum_from_non_string() {
            let value = Value::Integer(42);
            let result: Result<TestEnum, Error> = value.try_deserialize();
            assert!(result.is_err());
        }
    }
}
