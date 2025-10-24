use super::{
    expr::Expression,
    key::Key,
};
use crate::{
    Error,
    Map,
    Result,
    Value,
    errors::ValidationError,
};

impl Value {
    #[allow(clippy::needless_pass_by_value)]
    pub fn set<K: Key>(&mut self, key: K, value: Self) -> Result<&mut Self> {
        let expr = key.to_key()?;

        match expr {
            Expression::Identifier(id) => {
                match self {
                    Self::Table(table) => {
                        table.insert(id, value);
                    }
                    Self::Array(arr) => {
                        let raw_idx = id.parse::<isize>().map_err(|e| {
                            Error::ValidationError(ValidationError::new(
                                "set",
                                format!("Invalid array index '{id}': {e}"),
                            ))
                        })?;
                        if arr.is_empty() {
                            // Empty array: any index other than 0 becomes 0
                            // after normalization
                            let normalized =
                                if raw_idx < 0 { 0 } else { raw_idx as usize };
                            arr.resize(normalized.max(1), Self::Null);
                            arr[normalized] = value;
                        } else {
                            let normalized =
                                raw_idx.rem_euclid(arr.len() as isize) as usize;
                            arr.resize(normalized.max(arr.len()), Self::Null);
                            arr[normalized] = value;
                        }
                    }
                    _ => {
                        *self =
                            Self::Table([(id, value)].into_iter().collect());
                    }
                }
                Ok(self)
            }
            Expression::Subscript(id, idx) => {
                match self {
                    Self::Table(table) => {
                        let arr = if let Some(existing) = table.get_mut(&id) {
                            existing
                        } else {
                            table.insert(id.clone(), Self::Array(Vec::new()));
                            table.get_mut(&id).ok_or_else(|| {
                                Error::ValidationError(ValidationError::new(
                                    "set",
                                    "Failed to get mutable table entry",
                                ))
                            })?
                        };

                        if let Self::Array(arr) = arr {
                            if arr.is_empty() {
                                arr.push(Self::Null); // ensure at least one slot
                            }
                            let normalized =
                                idx.rem_euclid(arr.len() as isize) as usize;
                            arr.resize(normalized.max(arr.len()), Self::Null);
                            arr[normalized] = value;
                            Ok(self)
                        } else {
                            *arr = Self::Array(vec![value]);
                            Ok(self)
                        }
                    }
                    _ => Err(Error::ValidationError(ValidationError::new(
                        "set",
                        format!("Expected a table, got {}", self.value_type()),
                    ))),
                }
            }
            Expression::Child(exprs) => {
                let mut current = self;
                for (i, e) in exprs.iter().enumerate() {
                    if i == exprs.len() - 1 {
                        return current.set(e.clone(), value);
                    }
                    current = if let Self::Table(table) = current {
                        if !table.contains_key(&e.to_string()) {
                            table
                                .insert(e.to_string(), Self::Table(Map::new()));
                        }
                        table.get_mut(&e.to_string()).ok_or_else(|| {
                            Error::ValidationError(ValidationError::new(
                                "set",
                                "Failed to get child table",
                            ))
                        })?
                    } else {
                        let mut new_table = Map::new();
                        new_table
                            .insert(e.to_string(), Self::Table(Map::new()));
                        *current = Self::Table(new_table);
                        current.get_mut(e).ok_or_else(|| {
                            Error::ValidationError(ValidationError::new(
                                "set",
                                "Failed to get nested table",
                            ))
                        })?
                    };
                }
                Ok(current)
            }
        }
    }

    pub fn merge(&mut self, other: &Self) {
        match (self, other) {
            (Self::Table(a), Self::Table(b)) => {
                for (k, v) in b {
                    a.entry(k.clone())
                        .and_modify(|a| a.merge(v))
                        .or_insert(v.clone());
                }
            }
            (this, other) => *this = other.clone(),
        }
    }

    // pub fn with<K: Key + Clone, F>(&mut self, key: K, f: F) -> &mut Self
    // where
    //     F: FnOnce(&mut Self),
    // {
    //     let mut inner_value = self
    //         .get(key.clone())
    //         .unwrap_or_else(|| Self::Table(Map::new()));
    //     f(&mut inner_value);
    //     self.set(key, inner_value);
    //     self
    // }
}
