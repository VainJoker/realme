use std::{
    borrow::Borrow,
    collections::{
        HashMap,
        hash_map::Entry,
    },
    hash::Hash,
};

type InnerMap<K, V> = HashMap<K, V>;
type InnerEntry<'a, K, V> = Entry<'a, K, V>;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Map<K, V>
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

    #[inline]
    pub(crate) fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.inner.insert(k, v)
    }

    #[inline]
    pub(crate) fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.inner.get(k)
    }

    pub(crate) fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.inner.get_mut(k)
    }

    pub(crate) fn len(&self) -> usize {
        self.inner.len()
    }

    pub(crate) fn contains_key<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.inner.contains_key(k)
    }

    pub(crate) fn entry(&mut self, k: K) -> InnerEntry<'_, K, V> {
        self.inner.entry(k)
    }

    pub(crate) fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        <&Self as IntoIterator>::into_iter(self)
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
        Self {
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

impl<'a, K, V> IntoIterator for &'a Map<K, V>
where
    K: Hash + Eq,
{
    type Item = (&'a K, &'a V);
    type IntoIter = <&'a InnerMap<K, V> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}
