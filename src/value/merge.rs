use super::Value;

pub trait Merge {
    fn merge(&self, other: &Self) -> Self;
}

impl Merge for Value {
    fn merge(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Table(a), Self::Table(b)) => {
                let mut merged = a.clone();
                for (k, v) in b {
                    merged
                        .entry(k.clone())
                        .and_modify(|e| *e = e.merge(v))
                        .or_insert_with(|| v.clone());
                }
                Self::Table(merged)
            }
            (_, other) => other.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Map;

    #[test]
    fn test_merge() {
        let mut a_map = Map::new();
        a_map.insert("name".to_string(), Value::String("Tom".to_string()));
        a_map.insert(
            "dob".to_string(),
            Value::String("1979-05-27T07:32:00Z".to_string()),
        );
        let mut nested = Map::new();
        nested
            .insert("city".to_string(), Value::String("New York".to_string()));
        a_map.insert("address".to_string(), Value::Table(nested));

        let a = Value::Table(a_map);

        let mut b_map = Map::new();
        b_map.insert("name".to_string(), Value::String("Jasper".to_string()));
        let mut nested = Map::new();
        nested.insert(
            "city".to_string(),
            Value::String("San Francisco".to_string()),
        );
        nested.insert("zip".to_string(), Value::String("94105".to_string()));
        b_map.insert("address".to_string(), Value::Table(nested));

        let b = Value::Table(b_map);

        let merged = a.merge(&b);
        eprintln!("a: {a:#?}");
        eprintln!("b: {b:#?}");
        eprintln!("merged: {merged:#?}");

        if let Value::Table(merged_map) = merged {
            assert_eq!(
                merged_map.get("name"),
                Some(&Value::String("Jasper".to_string()))
            );
            assert_eq!(
                merged_map.get("dob"),
                Some(&Value::String("1979-05-27T07:32:00Z".to_string()))
            );
            if let Some(Value::Table(address)) = merged_map.get("address") {
                assert_eq!(
                    address.get("city"),
                    Some(&Value::String("San Francisco".to_string()))
                );
                assert_eq!(
                    address.get("zip"),
                    Some(&Value::String("94105".to_string()))
                );
            } else {
                panic!("Expected nested address table");
            }
        } else {
            panic!("Expected merged result to be a table");
        }
    }
}
