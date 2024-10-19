use std::{
    borrow::Borrow,
    collections::HashMap,
    hash::Hash,
};

type InnerMap<K, V> = HashMap<K, V>;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Map<K, V>
where
    K: Hash + Eq,
{
    inner: InnerMap<K, V>,
}

impl<K, V> Map<K, V>
where
    K: Hash + Eq,
{
    pub(crate) fn new() -> Self {
        Self {
            inner: InnerMap::new(),
        }
    }

    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: InnerMap::with_capacity(capacity),
        }
    }

    pub(crate) fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.inner.insert(k, v)
    }

    pub(crate) fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.inner.get(k)
    }

    pub(crate) fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.inner.get_mut(k)
    }

    pub(crate) fn len(&self) -> usize {
        self.inner.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub(crate) fn clear(&mut self) {
        self.inner.clear()
    }

    pub(crate) fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.inner.contains_key(k)
    }
}

impl<K, V> FromIterator<(K, V)> for Map<K, V>
where
    K: Hash + Eq,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (K, V)>,
    {
        Map {
            inner: FromIterator::from_iter(iter),
        }
    }
}

impl<K, V> IntoIterator for Map<K, V>
where
    K: Hash + Eq,
{
    type Item = (K, V);
    type IntoIter = <InnerMap<K, V> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
