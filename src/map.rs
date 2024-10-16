use std::{
    borrow::Borrow,
    collections::hash_map::{Entry, HashMap, Iter},
    hash::Hash,
};

#[derive(Debug, Clone, Default)]
pub struct Map<K, V> {
    inner: HashMap<K, V>,
}

impl<K: Eq + Hash, V> Map<K, V> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: HashMap::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.inner.insert(key, value)
    }

    #[allow(clippy::multiple_bound_locations)]
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.inner.get(key)
    }

    #[allow(clippy::multiple_bound_locations)]
    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.inner.get_mut(key)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        self.inner.iter()
    }

    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        self.inner.entry(key)
    }
}

impl<K: Eq + Hash, V> FromIterator<(K, V)> for Map<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self {
            inner: iter.into_iter().collect(),
        }
    }
}

impl<K: Eq + Hash, V, const N: usize> From<[(K, V); N]> for Map<K, V> {
    fn from(arr: [(K, V); N]) -> Self {
        Self {
            inner: arr.into_iter().collect(),
        }
    }
}

impl<K, V> IntoIterator for Map<K, V> {
    type Item = (K, V);
    type IntoIter = std::collections::hash_map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, K, V> IntoIterator for &'a Map<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<K: Eq + Hash, V: PartialEq> PartialEq for Map<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}
