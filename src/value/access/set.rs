use super::{
    expr::Expression,
    key::Key,
};
use crate::{
    Error,
    Map,
    Result,
    Value,
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
                        let idx = id
                            .parse::<isize>()
                            .map_err(|e| Error::SetValueError(e.to_string()))?;
                        let idx = idx.rem_euclid(arr.len() as isize) as usize;
                        // if idx >= arr.len() {
                        //     return Err(Error::SetValueError(format!("Index {}
                        // out of bounds for array of length {}", idx,
                        // arr.len()))); }
                        arr.resize(idx.max(arr.len()), Self::Null);
                        arr[idx] = value;
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
                            table.get_mut(&id).expect("Failed to get mut table")
                        };

                        if let Self::Array(arr) = arr {
                            let idx =
                                idx.rem_euclid(arr.len() as isize) as usize;
                            // if idx >= arr.len() {
                            //     return
                            // Err(Error::SetValueError(format!("Index {} out of
                            // bounds for array of length {}", idx,
                            // arr.len()))); }
                            arr.resize(idx.max(arr.len()), Self::Null);
                            arr[idx] = value;
                            Ok(self)
                        } else {
                            *arr = Self::Array(vec![value]);
                            Ok(self)
                        }
                    }
                    _ => Err(Error::SetValueError(format!(
                        "Expected a table, got {}",
                        self.value_type()
                    ))),
                }
            }
            Expression::Child(exprs) => {
                let mut current = self;
                for (i, e) in exprs.iter().enumerate() {
                    if i == exprs.len() - 1 {
                        return current.set(e.to_owned(), value);
                    }
                    current = if let Self::Table(table) = current {
                        if !table.contains_key(&e.to_string()) {
                            table
                                .insert(e.to_string(), Self::Table(Map::new()));
                        }
                        table
                            .get_mut(&e.to_string())
                            .expect("Failed to get mut table")
                    } else {
                        let mut new_table = Map::new();
                        new_table
                            .insert(e.to_string(), Self::Table(Map::new()));
                        *current = Self::Table(new_table);
                        current.get_mut(e).expect("Failed to get mut table")
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
                    if let Some(existing) = a.get_mut(k) {
                        existing.merge(v);
                    } else {
                        a.insert(k.clone(), v.clone());
                    }
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
