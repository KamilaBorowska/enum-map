use EnumMap;
use Internal;

use core::hash::{Hash, Hasher};
use core::ops::{Index, IndexMut};

impl<K: Internal<V>, V: Default> EnumMap<K, V>
    where K::Array: Default
{
    /// Creates an enum map with default values.
    ///
    /// ```
    /// extern crate enum_map;
    /// #[macro_use]
    /// extern crate enum_map_derive;
    ///
    /// use enum_map::EnumMap;
    ///
    /// #[derive(EnumMap)]
    /// enum Example {
    ///     A,
    /// }
    ///
    /// fn main() {
    ///     let enum_map = EnumMap::<_, i32>::new();
    ///     assert_eq!(enum_map[Example::A], 0);
    /// }
    /// ```
    pub fn new() -> Self {
        EnumMap::default()
    }
}

// Implementations provided by derive attribute are too specific, and put requirements on K.
// This is caused by rust-lang/rust#26925.
impl<K: Internal<V>, V> Clone for EnumMap<K, V>
    where K::Array: Clone
{
    fn clone(&self) -> Self {
        EnumMap { array: self.array.clone() }
    }
}

impl<K: Internal<V>, V> Copy for EnumMap<K, V> where K::Array: Copy {}

impl<K: Internal<V>, V> PartialEq for EnumMap<K, V>
    where K::Array: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.array == other.array
    }
}

impl<K: Internal<V>, V> Eq for EnumMap<K, V> where K::Array: Eq {}

impl<K: Internal<V>, V> Hash for EnumMap<K, V>
    where K::Array: Hash
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.array.hash(state);
    }
}

impl<K: Internal<V>, V> Default for EnumMap<K, V>
    where K::Array: Default
{
    fn default() -> Self {
        EnumMap { array: K::Array::default() }
    }
}

impl<K: Internal<V>, V> Index<K> for EnumMap<K, V> {
    type Output = V;

    fn index(&self, key: K) -> &V {
        &K::slice(&self.array)[key.to_usize()]
    }
}

impl<K: Internal<V>, V> IndexMut<K> for EnumMap<K, V> {
    fn index_mut(&mut self, key: K) -> &mut V {
        &mut K::slice_mut(&mut self.array)[key.to_usize()]
    }
}
