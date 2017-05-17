//! An enum mapping type.
//!
//! It is implemented using array, so using it is as fast as using Rust
//! arrays.

#![no_std]
#![deny(missing_docs)]

use core::hash::{Hash, Hasher};
use core::iter::Enumerate;
use core::marker::PhantomData;
use core::ops::{Index, IndexMut};
use core::slice;

// this is not stable, do not depend on this
#[doc(hidden)]
pub trait Internal<V>: Sized {
    type Array;
    fn slice(&Self::Array) -> &[V];
    fn slice_mut(&mut Self::Array) -> &mut [V];
    fn from_usize(usize) -> Self;
    fn to_usize(self) -> usize;
    fn from_function<F: Fn(Self) -> V>(F) -> Self::Array;
}

/// An enum mapping.
///
/// This internally uses an array which stores a value for each possible
/// enum value. To work, it requires implementation of internal (private,
/// although public due to macro limitations) trait which allows extracting
/// information about an enum, which can be automatically generated using
/// `#[derive(EnumMap)]` from `enum_map_derive` crate.
///
/// # Examples
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
///     B,
///     C,
/// }
///
/// fn main() {
///     let mut map = EnumMap::new();
///     // new initializes map with default values
///     assert_eq!(map[Example::A], 0);
///     map[Example::A] = 3;
///     assert_eq!(map[Example::A], 3);
/// }
/// ```
#[derive(Debug)]
pub struct EnumMap<K: Internal<V>, V> {
    array: K::Array,
}

impl<K: Internal<V>, V> EnumMap<K, V> {
    /// Returns an iterator over enum map.
    pub fn iter(&self) -> Iter<K, V> {
        self.into_iter()
    }

    /// Returns a mutable iterator over enum map.
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        self.into_iter()
    }
}

impl<F: Fn(K) -> V, K: Internal<V>, V> From<F> for EnumMap<K, V> {
    fn from(f: F) -> Self {
        EnumMap { array: K::from_function(f) }
    }
}

/// Enum map constructor.
///
/// This macro allows to create a new enum map in a type safe way. It takes
/// a list of `,` separated pairs separated by `=>`. Left side is `|`
/// separated list of enum keys, or `_` to match all unmatched enum keys,
/// while right side is a value.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate enum_map;
/// #[macro_use]
/// extern crate enum_map_derive;
///
/// #[derive(EnumMap)]
/// enum Example {
///     A,
///     B,
///     C,
///     D,
/// }
///
/// fn main() {
///     let enum_map = enum_map! {
///         Example::A | Example::B => 1,
///         Example::C => 2,
///         _ => 3,
///     };
///     assert_eq!(enum_map[Example::A], 1);
///     assert_eq!(enum_map[Example::B], 1);
///     assert_eq!(enum_map[Example::C], 2);
///     assert_eq!(enum_map[Example::D], 3);
/// }
/// ```
#[macro_export]
macro_rules! enum_map {
    {$($t:tt)*} => {
        ::enum_map::EnumMap::from(|k| match k { $($t)* })
    };
}

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

/// Immutable enum map iterator
///
/// This struct is created by `iter` method or `into_iter` on a reference
/// to `EnumMap`.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate enum_map;
/// #[macro_use]
/// extern crate enum_map_derive;
///
/// #[derive(EnumMap)]
/// enum Example {
///     A,
///     B,
///     C,
/// }
///
/// fn main() {
///     let mut map = enum_map! { Example::A => 3, _ => 0 };
///     assert_eq!(map[Example::A], 3);
///     for (key, &value) in &map {
///         assert_eq!(match key {
///             Example::A => 3,
///             _ => 0,
///         }, value);
///     }
/// }
/// ```
pub struct Iter<'a, K, V: 'a> {
    _phantom: PhantomData<K>,
    iterator: Enumerate<slice::Iter<'a, V>>,
}

impl<'a, K: Internal<V>, V> Iterator for Iter<'a, K, V> {
    type Item = (K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .next()
            .map(|(index, item)| (K::from_usize(index), item))
    }
}

impl<'a, K: Internal<V>, V> IntoIterator for &'a EnumMap<K, V> {
    type Item = (K, &'a V);
    type IntoIter = Iter<'a, K, V>;
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            _phantom: PhantomData,
            iterator: K::slice(&self.array).iter().enumerate(),
        }
    }
}

/// Mutable map iterator
///
/// This struct is created by `iter_mut` method or `into_iter` on a mutable
/// reference to `EnumMap`.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate enum_map;
/// #[macro_use]
/// extern crate enum_map_derive;
///
/// #[derive(Debug, EnumMap)]
/// enum Example {
///     A,
///     B,
///     C,
/// }
///
/// fn main() {
///     let mut map = enum_map! { Example::A => 3, _ => 0 };
///     for (_, value) in &mut map {
///         *value += 1;
///     }
///     assert_eq!(map, enum_map! { Example::A => 4, _ => 1 });
/// }
/// ```
pub struct IterMut<'a, K, V: 'a> {
    _phantom: PhantomData<K>,
    iterator: Enumerate<slice::IterMut<'a, V>>,
}

impl<'a, K: Internal<V>, V> Iterator for IterMut<'a, K, V> {
    type Item = (K, &'a mut V);
    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .next()
            .map(|(index, item)| (K::from_usize(index), item))
    }
}

impl<'a, K: Internal<V>, V> IntoIterator for &'a mut EnumMap<K, V> {
    type Item = (K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;
    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            _phantom: PhantomData,
            iterator: K::slice_mut(&mut self.array).iter_mut().enumerate(),
        }
    }
}
