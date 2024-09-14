#[derive(Debug, Clone)]
pub struct Map<K, V> {
    inner: MapImpl<K, V>,
}
impl<K, V> Default for Map<K, V> {
    fn default() -> Self {
        Self {
            inner: MapImpl::new(),
        }
    }
}

type MapImpl<K, V> = std::collections::HashMap<K, V>;
type MapEntry<'a, K, V> = std::collections::hash_map::Entry<'a, K, V>;

impl<K, V> PartialEq for Map<K, V>
where
    K: Eq + std::hash::Hash,
    V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<K, V> FromIterator<(K, V)> for Map<K, V>
where
    K: Eq + std::hash::Hash,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self {
            inner: iter.into_iter().collect(),
        }
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for Map<K, V>
where
    K: Eq + std::hash::Hash,
{
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
    type IntoIter = std::collections::hash_map::Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<K, V> Map<K, V> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: std::hash::Hash + Eq,
    {
        self.inner.insert(key, value)
    }

    pub fn get(&self, key: &K) -> Option<&V>
    where
        K: std::hash::Hash + Eq,
    {
        self.inner.get(key)
    }

    pub fn collect<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Eq + std::hash::Hash,
    {
        Self {
            inner: iter.into_iter().collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<K, V> {
        <&Self as IntoIterator>::into_iter(self)
    }

    pub fn entry(&mut self, key: K) -> MapEntry<K, V>
    where
        K: Eq + std::hash::Hash,
    {
        self.inner.entry(key)
    }

    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Eq + std::hash::Hash,
    {
        Self {
            inner: iter.into_iter().collect(),
        }
    }
}
