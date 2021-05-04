//! An enum mapping type.
//!
//! It is implemented using an array type, so using it is as fast as using Rust
//! arrays.
//!
//! # Examples
//!
//! ```
//! use enum_map::{enum_map, Enum, EnumMap};
//!
//! #[derive(Debug, Enum)]
//! enum Example {
//!     A,
//!     B,
//!     C,
//! }
//!
//! let mut map = enum_map! {
//!     Example::A => 1,
//!     Example::B => 2,
//!     Example::C => 3,
//! };
//! map[Example::C] = 4;
//!
//! assert_eq!(map[Example::A], 1);
//!
//! for (key, &value) in &map {
//!     println!("{:?} has {} as value.", key, value);
//! }
//! ```

#![no_std]
#![deny(missing_docs)]

mod enum_map_impls;
mod internal;
mod iter;
mod serde;

#[doc(hidden)]
pub use core::mem::{ManuallyDrop, MaybeUninit};
#[doc(hidden)]
pub use core::ptr;
pub use enum_map_derive::Enum;
use internal::Array;
pub use internal::Enum;
pub use iter::{IntoIter, Iter, IterMut, Values, ValuesMut};

// Type invariant: arr[..len] must be initialized
#[doc(hidden)]
#[non_exhaustive]
pub struct ArrayVec<K, V>
where
    K: Enum<V>,
{
    pub array: MaybeUninit<K::Array>,
    pub length: usize,
}

impl<K, V> ArrayVec<K, V>
where
    K: Enum<V>,
{
    #[doc(hidden)]
    // This function is marked as unsafe to prevent user from causing unsafety
    // by using undocumented ArrayVec.
    pub unsafe fn new() -> Self {
        ArrayVec {
            array: MaybeUninit::uninit(),
            length: 0,
        }
    }

    #[doc(hidden)]
    pub fn storage_length(&self) -> usize {
        K::Array::LENGTH
    }

    #[doc(hidden)]
    pub fn get_key(&self) -> K {
        K::from_usize(self.length)
    }

    #[doc(hidden)]
    // Unsafe as it can write out of bounds.
    pub unsafe fn push(&mut self, value: V) {
        self.array
            .as_mut_ptr()
            .cast::<V>()
            .add(self.length)
            .write(value);
        self.length += 1;
    }
}

impl<K, V> Drop for ArrayVec<K, V>
where
    K: Enum<V>,
{
    fn drop(&mut self) {
        // This is safe as arr[..len] is initialized due to
        // __ArrayVecInner's type invariant.
        unsafe {
            ptr::slice_from_raw_parts_mut(self.array.as_mut_ptr() as *mut V, self.length)
                .drop_in_place();
        }
    }
}

#[doc(hidden)]
pub union TypeEqualizer<K, V>
where
    K: Enum<V>,
{
    pub init: (),
    pub enum_map: ManuallyDrop<EnumMap<K, V>>,
    pub array_vec: ManuallyDrop<ArrayVec<K, V>>,
}

/// Enum map constructor.
///
/// This macro allows to create a new enum map in a type safe way. It takes
/// a list of `,` separated pairs separated by `=>`. Left side is `|`
/// separated list of enum keys, or `_` to match all unmatched enum keys,
/// while right side is a value.
///
/// The iteration order when using this macro is not guaranteed to be
/// consistent. Future releases of this crate may change it, and this is not
/// considered to be a breaking change.
///
/// # Examples
///
/// ```
/// # extern crate enum_map;
/// use enum_map::{enum_map, Enum};
///
/// #[derive(Enum)]
/// enum Example {
///     A,
///     B,
///     C,
///     D,
/// }
///
/// let enum_map = enum_map! {
///     Example::A | Example::B => 1,
///     Example::C => 2,
///     _ => 3,
/// };
/// assert_eq!(enum_map[Example::A], 1);
/// assert_eq!(enum_map[Example::B], 1);
/// assert_eq!(enum_map[Example::C], 2);
/// assert_eq!(enum_map[Example::D], 3);
/// ```
#[macro_export]
macro_rules! enum_map {
    {$($t:tt)*} => {{
        let mut type_equalizer = $crate::TypeEqualizer { init: () };
        if false {
            // Safe because this code is unreachable
            unsafe {
                type_equalizer.enum_map = $crate::MaybeUninit::assume_init($crate::MaybeUninit::uninit());
                $crate::ManuallyDrop::into_inner(type_equalizer.enum_map)
            }
        } else {
            // Safe because we are going to follow ArrayVec invariant
            type_equalizer.array_vec = $crate::ManuallyDrop::new(unsafe { $crate::ArrayVec::new() });
            // Safe because we just wrote to array_vec field.
            let mut vec = $crate::ManuallyDrop::into_inner(unsafe { type_equalizer.array_vec });
            for _ in 0..$crate::ArrayVec::storage_length(&vec) {
                let _please_do_not_use_continue_without_label;
                let value;
                struct __PleaseDoNotUseBreakWithoutLabel;
                #[allow(unreachable_code)]
                loop {
                    _please_do_not_use_continue_without_label = ();
                    value = match $crate::ArrayVec::get_key(&vec) { $($t)* };
                    break __PleaseDoNotUseBreakWithoutLabel;
                };
                // Safe because this method will be called at most storage_length times.
                unsafe { $crate::ArrayVec::push(&mut vec, value); }
            }
            vec.length = 0;
            // Safe because the array was fully initialized.
            $crate::EnumMap::from_array(unsafe { $crate::ptr::read($crate::MaybeUninit::as_ptr(&vec.array)) })
        }
    }};
}

/// An enum mapping.
///
/// This internally uses an array which stores a value for each possible
/// enum value. To work, it requires implementation of internal (private,
/// although public due to macro limitations) trait which allows extracting
/// information about an enum, which can be automatically generated using
/// `#[derive(Enum)]` macro.
///
/// Additionally, `bool` and `u8` automatically derives from `Enum`. While
/// `u8` is not technically an enum, it's convenient to consider it like one.
/// In particular, [reverse-complement in benchmark game] could be using `u8`
/// as an enum.
///
/// # Examples
///
/// ```
/// # extern crate enum_map;
/// use enum_map::{enum_map, Enum, EnumMap};
///
/// #[derive(Enum)]
/// enum Example {
///     A,
///     B,
///     C,
/// }
///
/// let mut map = EnumMap::default();
/// // new initializes map with default values
/// assert_eq!(map[Example::A], 0);
/// map[Example::A] = 3;
/// assert_eq!(map[Example::A], 3);
/// ```
///
/// [reverse-complement in benchmark game]:
///     http://benchmarksgame.alioth.debian.org/u64q/program.php?test=revcomp&lang=rust&id=2
pub struct EnumMap<K: Enum<V>, V> {
    array: K::Array,
}

impl<K: Enum<V>, V: Default> EnumMap<K, V> {
    /// Clear enum map with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate enum_map;
    /// use enum_map::{Enum, EnumMap};
    ///
    /// #[derive(Enum)]
    /// enum Example {
    ///     A,
    ///     B,
    /// }
    ///
    /// let mut enum_map = EnumMap::<_, String>::default();
    /// enum_map[Example::B] = "foo".into();
    /// enum_map.clear();
    /// assert_eq!(enum_map[Example::A], "");
    /// assert_eq!(enum_map[Example::B], "");
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        for v in self.as_mut_slice() {
            *v = V::default();
        }
    }
}

#[allow(clippy::len_without_is_empty)]
impl<K: Enum<V>, V> EnumMap<K, V> {
    /// Creates an enum map from array.
    #[inline]
    pub fn from_array(array: K::Array) -> EnumMap<K, V> {
        EnumMap { array }
    }

    /// Returns an iterator over enum map.
    #[inline]
    pub fn iter(&self) -> Iter<K, V> {
        self.into_iter()
    }

    /// Returns a mutable iterator over enum map.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        self.into_iter()
    }

    /// Returns number of elements in enum map.
    #[inline]
    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    /// Swaps two indexes.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate enum_map;
    /// use enum_map::enum_map;
    ///
    /// let mut map = enum_map! { false => 0, true => 1 };
    /// map.swap(false, true);
    /// assert_eq!(map[false], 1);
    /// assert_eq!(map[true], 0);
    /// ```
    #[inline]
    pub fn swap(&mut self, a: K, b: K) {
        self.as_mut_slice().swap(a.into_usize(), b.into_usize())
    }

    /// Converts an enum map to a slice representing values.
    #[inline]
    pub fn as_slice(&self) -> &[V] {
        self.array.slice()
    }

    /// Converts a mutable enum map to a mutable slice representing values.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [V] {
        self.array.slice_mut()
    }
}

#[cfg(feature = "arbitrary")]
impl<'a, K: Enum<V>, V: arbitrary::Arbitrary<'a>> arbitrary::Arbitrary<'a> for EnumMap<K, V> {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<EnumMap<K, V>> {
        Ok(enum_map! {
            _ => arbitrary::Arbitrary::arbitrary(u)?,
        })
    }
}
