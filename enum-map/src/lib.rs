//! An enum mapping type.
//!
//! It is implemented using an array type, so using it is as fast as using Rust
//! arrays.
//!
//! # Examples
//!
//! ```
//! #[macro_use]
//! extern crate enum_map;
//! #[macro_use]
//! extern crate enum_map_derive;
//!
//! use enum_map::EnumMap;
//!
//! #[derive(Debug, EnumMap)]
//! enum Example {
//!     A,
//!     B,
//!     C,
//! }
//!
//! fn main() {
//!     let mut map = enum_map! {
//!         Example::A => 1,
//!         Example::B => 2,
//!         Example::C => 3,
//!     };
//!     map[Example::C] = 4;
//!
//!     assert_eq!(map[Example::A], 1);
//!
//!     for (key, &value) in &map {
//!         println!("{:?} has {} as value.", key, value);
//!     }
//! }
//! ```

#![no_std]
#![deny(missing_docs)]

use core::iter::Enumerate;
use core::marker::PhantomData;
use core::slice;

mod enummap_impls;
mod implementations;
mod serde;

// this is not stable, do not depend on this
#[doc(hidden)]
pub trait Internal<V>: Sized {
    type Array;
    fn slice(&Self::Array) -> &[V];
    fn slice_mut(&mut Self::Array) -> &mut [V];
    fn from_usize(usize) -> Self;
    fn to_usize(self) -> usize;
    fn from_function<F: FnMut(Self) -> V>(F) -> Self::Array;
}

/// An enum mapping.
///
/// This internally uses an array which stores a value for each possible
/// enum value. To work, it requires implementation of internal (private,
/// although public due to macro limitations) trait which allows extracting
/// information about an enum, which can be automatically generated using
/// `#[derive(EnumMap)]` from `enum_map_derive` crate.
///
/// Additionally, `bool` and `u8` automatically derives from `EnumMap`. While
/// `u8` is not technically an enum, it's convenient to consider it like one.
/// In particular, [reverse-complement in benchmark game] could be using `u8`
/// as an enum.
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
///
/// [reverse-complement in benchmark game]:
///     http://benchmarksgame.alioth.debian.org/u64q/program.php?test=revcomp&lang=rust&id=2
#[derive(Debug)]
pub struct EnumMap<K: Internal<V>, V> {
    array: K::Array,
}

impl<K: Internal<V>, V> EnumMap<K, V> {
    /// Returns number of elements in enum map.
    pub fn len(&self) -> usize {
        K::slice(&self.array).len()
    }

    /// Returns whether the enum variant set is empty.
    ///
    /// This isn't particularly useful, as there is no real reason to use
    /// enum map for enums without variants. However, it is provided for
    /// consistency with data structures providing len method (and I will
    /// admit, to avoid clippy warnings).
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
    /// enum Void {}
    ///
    /// #[derive(EnumMap)]
    /// enum SingleVariant {
    ///     Variant,
    /// }
    ///
    /// fn main() {
    ///     assert_eq!(EnumMap::<Void, ()>::new().is_empty(), true);
    ///     assert_eq!(EnumMap::<SingleVariant, ()>::new().is_empty(), false);
    /// }
    pub fn is_empty(&self) -> bool {
        K::slice(&self.array).is_empty()
    }

    /// Returns an iterator over enum map.
    pub fn iter(&self) -> Iter<K, V> {
        self.into_iter()
    }

    /// Returns a mutable iterator over enum map.
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        self.into_iter()
    }
}

impl<F: FnMut(K) -> V, K: Internal<V>, V> From<F> for EnumMap<K, V> {
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iterator.size_hint()
    }
}

impl<'a, K: Internal<V>, V> DoubleEndedIterator for Iter<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iterator
            .next_back()
            .map(|(index, item)| (K::from_usize(index), item))
    }
}

impl<'a, K: Internal<V>, V> ExactSizeIterator for Iter<'a, K, V> {}

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

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iterator.size_hint()
    }
}

impl<'a, K: Internal<V>, V> DoubleEndedIterator for IterMut<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iterator
            .next_back()
            .map(|(index, item)| (K::from_usize(index), item))
    }
}

impl<'a, K: Internal<V>, V> ExactSizeIterator for IterMut<'a, K, V> {}

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
