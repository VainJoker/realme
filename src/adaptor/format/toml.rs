use crate::{errors::RealmError, parser::Parser};

// pub fn parse(content: &str) -> Result<Value, RealmError> {
//     let value: toml::Value =
//         toml::from_str(content).map_err(|e| anyhow::anyhow!(e))?;
//     Ok(value.into())
// }

// impl From<toml::Value> for Value {
//     fn from(value: toml::Value) -> Self {
//         match value {
//             toml::Value::String(s) => Self::String(s),
//             toml::Value::Integer(i) => Self::Integer(i),
//             toml::Value::Float(f) => Self::Float(f),
//             toml::Value::Boolean(b) => Self::Boolean(b),
//             // TODO: datetime
//             toml::Value::Datetime(d) => Self::String(d.to_string()),
//             toml::Value::Array(a) => Self::Array(
//                 a.into_iter().map(std::convert::Into::into).collect(),
//             ),
//             toml::Value::Table(t) => Self::Table(
//                 t.into_iter()
//                     .map(|(k, v)| (k, v.into()))
//                     .collect::<Map<String, Self>>(),
//             ),
//         }
//     }
// }

#[derive(Debug)]
pub struct TomlParser;

impl Parser for TomlParser {
    type Item = toml::Value;

    type Error = RealmError;

    fn parse(content: &str) -> Result<Self::Item, Self::Error> {
        let value: toml::Value =
            toml::from_str(content).map_err(|e| anyhow::anyhow!(e))?;
        Ok(value)
    }
}

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn test_parse_toml() {
//         let toml = r#"
//        # 这是一个注释

// # 标题
// [owner]
// name = "Tom Preston-Werner"
// dob = 1979-05-27T07:32:00Z

// # 这是一个数组
// [database]
// server = "192.168.1.1"
// ports = [ 8001, 8002, 8003 ]
// connection_max = 5000
// enabled = true

// # 嵌套表
// [servers]
//   [servers.alpha]
//   ip = "10.0.0.1"
//   dc = "eqdc1"

//   [servers.beta]
//   ip = "10.0.0.2"
//   dc = "eqdc2"

// # 表格标题
// [products]
//   name = "A"
//   description = "A product"
//   price = 19.99

//   [products.features]
//   color = "red"
//   size = "medium"

//   # 子表
//   [[products.reviews]]
//   reviewer = "John"
//   comment = "Great product!"

//   [[products.reviews]]
//   reviewer = "Jane"
//   comment = "Not bad."

// # 日期时间
// [logs]
// date_format = "2006-01-02T15:04:05Z07:00"

// # 浮点数
// [metrics]
// ratio = 0.5
// threshold = 1.0

// # 字符串
// [settings]
// title = "Sample Config"
// description = 'This is a "description" with escaped quotes.'

// # 自定义时间
// [custom]
// time = 2024-09-01T13:45:30

// # 表达式
// [[expressions]]
// name = "simple"
// value = "2 + 2"

// [[expressions]]
// name = "complex"
// value = "sin(x) * cos(y)"

//         "#;

//         let t_value: toml::Value = toml::from_str(toml).unwrap();
//         let value: Value = t_value.into();
//         println!("{:#?}", value.get("owner").and_then(|v| v.get("name")));
//         println!("{:#?}", value.get("owner").and_then(|v| v.get("name2")));
//         println!("{:#?}", value.get("owner2").and_then(|v| v.get("name")));
//         println!("{:#?}", value.get("owner2").and_then(|v| v.get("name2")));
//     }
// }
