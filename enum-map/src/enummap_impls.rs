use EnumMap;
use Internal;

use core::iter::Extend;
use core::hash::{Hash, Hasher};
use core::ops::{Index, IndexMut};

impl<K: Internal<V>, V> Extend<(K, V)> for EnumMap<K, V> {
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        for (key, value) in iter {
            self[key] = value;
        }
    }
}

impl<'a, K, V> Extend<(&'a K, &'a V)> for EnumMap<K, V>
where
    K: Internal<V> + Copy,
    V: Copy,
{
    fn extend<I: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: I) {
        self.extend(iter.into_iter().map(|(&key, &value)| (key, value)))
    }
}

impl<K: Internal<V>, V> Index<K> for EnumMap<K, V> {
    type Output = V;

    fn index(&self, key: K) -> &V {
        &self.as_slice()[key.to_usize()]
    }
}

impl<K: Internal<V>, V> IndexMut<K> for EnumMap<K, V> {
    fn index_mut(&mut self, key: K) -> &mut V {
        &mut self.as_mut_slice()[key.to_usize()]
    }
}


// Implementations provided by derive attribute are too specific, and put requirements on K.
// This is caused by rust-lang/rust#26925.
impl<K: Internal<V>, V> Clone for EnumMap<K, V>
where
    K::Array: Clone,
{
    fn clone(&self) -> Self {
        EnumMap { array: self.array.clone() }
    }
}

impl<K: Internal<V>, V> Copy for EnumMap<K, V>
where
    K::Array: Copy,
{
}

impl<K: Internal<V>, V> PartialEq for EnumMap<K, V>
where
    K::Array: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.array == other.array
    }
}

impl<K: Internal<V>, V> Eq for EnumMap<K, V>
where
    K::Array: Eq,
{
}

impl<K: Internal<V>, V> Hash for EnumMap<K, V>
where
    K::Array: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.array.hash(state);
    }
}

impl<K: Internal<V>, V> Default for EnumMap<K, V>
where
    K::Array: Default,
{
    fn default() -> Self {
        EnumMap { array: K::Array::default() }
    }
}
