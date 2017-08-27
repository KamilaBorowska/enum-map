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

#[macro_use]
extern crate array_macro;
// This allows to quiet "proc macro crates and `#[no_link]` crates have no
// effect without `#[macro_use]`" warning. Just using extern DOES have
// an effect of letting me export the macro.
#[allow(unused_imports)]
#[macro_use]
extern crate enum_map_derive;

mod enummap_impls;
mod internal;
mod iter;
mod serde;

pub use internal::Internal;
pub use iter::{Iter, IterMut};
// `*` here means re-exporting a derive procedural macro.
pub use enum_map_derive::*;

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
/// #[macro_use]
/// extern crate enum_map;
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

impl<K: Internal<V>, V: Default> EnumMap<K, V>
where
    K::Array: Default,
{
    /// Creates an enum map with default values.
    ///
    /// ```
    /// #[macro_use]
    /// extern crate enum_map;
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
    /// #[macro_use]
    /// extern crate enum_map;
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

    /// Swaps two indexes.
    ///
    /// # Examples
    ///
    /// ```
    /// #[macro_use]
    /// extern crate enum_map;
    ///
    /// fn main() {
    ///     let mut map = enum_map! { false => 0, true => 1 };
    ///     map.swap(false, true);
    ///     assert_eq!(map[false], 1);
    ///     assert_eq!(map[true], 0);
    /// }
    /// ```
    pub fn swap(&mut self, a: K, b: K) {
        K::slice_mut(&mut self.array).swap(a.to_usize(), b.to_usize())
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
        $crate::EnumMap::from(|k| match k { $($t)* })
    };
}
