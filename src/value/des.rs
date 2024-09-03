// use std::fmt;

// use serde::{
//     de::{self, Deserializer, Visitor},
//     Deserialize,
// };

// use super::Value;

// impl<'de> Deserialize<'de> for Value {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         struct ValueVisitor;

//         impl<'de> Visitor<'de> for ValueVisitor {
//             type Value = Value;

//             fn expecting(&self, formatter: &mut fmt::Formatter) ->
// fmt::Result {                 formatter.write_str("a valid Value")
//             }

//             fn visit_str<E>(self, value: &str) -> Result<Value, E>
//             where
//                 E: de::Error,
//             {
//                 // 根据你的需求解析字符串
//                 Ok(Value::String(value.to_string()))
//             }

//             fn visit_i64<E>(self, value: i64) -> Result<Value, E>
//             where
//                 E: de::Error,
//             {
//                 Ok(Value::Integer(value))
//             }

//             // 你可以根据需要添加更多的 `visit_*` 方法
//         }

//         deserializer.deserialize_any(ValueVisitor)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_deserialize() {
//         let value: Value = serde_json::from_str("\"hello\"").unwrap();
//         assert_eq!(value, Value::String("hello".to_string()));
//     }
// }
