use EnumMap;
use Internal;

use core::iter::Enumerate;
use core::marker::PhantomData;
use core::slice;

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
///         assert_eq!(value, match key {
///             Example::A => 3,
///             _ => 0,
///         });
///     }
/// }
/// ```
#[derive(Debug)]
pub struct Iter<'a, K, V: 'a> {
    _phantom: PhantomData<K>,
    iterator: Enumerate<slice::Iter<'a, V>>,
}

impl<'a, K: Internal<V>, V> Iterator for Iter<'a, K, V> {
    type Item = (K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().map(|(index, item)| {
            (K::from_usize(index), item)
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iterator.size_hint()
    }
}

impl<'a, K: Internal<V>, V> DoubleEndedIterator for Iter<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iterator.next_back().map(|(index, item)| {
            (K::from_usize(index), item)
        })
    }
}

impl<'a, K: Internal<V>, V> ExactSizeIterator for Iter<'a, K, V> {}

impl<'a, K: Internal<V>, V> IntoIterator for &'a EnumMap<K, V> {
    type Item = (K, &'a V);
    type IntoIter = Iter<'a, K, V>;
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            _phantom: PhantomData,
            iterator: self.as_slice().iter().enumerate(),
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
#[derive(Debug)]
pub struct IterMut<'a, K, V: 'a> {
    _phantom: PhantomData<K>,
    iterator: Enumerate<slice::IterMut<'a, V>>,
}

impl<'a, K: Internal<V>, V> Iterator for IterMut<'a, K, V> {
    type Item = (K, &'a mut V);
    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().map(|(index, item)| {
            (K::from_usize(index), item)
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iterator.size_hint()
    }
}

impl<'a, K: Internal<V>, V> DoubleEndedIterator for IterMut<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iterator.next_back().map(|(index, item)| {
            (K::from_usize(index), item)
        })
    }
}

impl<'a, K: Internal<V>, V> ExactSizeIterator for IterMut<'a, K, V> {}

impl<'a, K: Internal<V>, V> IntoIterator for &'a mut EnumMap<K, V> {
    type Item = (K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;
    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            _phantom: PhantomData,
            iterator: self.as_mut_slice().iter_mut().enumerate(),
        }
    }
}
