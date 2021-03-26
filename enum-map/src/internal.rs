use array_macro::array;
use core::convert::Infallible;

/// Enum mapping type
///
/// This trait is internally used by `#[derive(Enum)]`. `Enum<T>` is
/// implemented by any enum type where V is a generic type representing a
/// value. The purpose of this generic type is to allow providing a value
/// type for arrays, as Rust currently does not support higher kinded types.
///
/// This trait is also implemented by `bool` and `u8`. While `u8` is
/// strictly speaking not an actual enum, there are good reasons to consider
/// it like one, as array of `u8` keys is a relatively common pattern.
pub trait Enum<V>: Sized {
    /// Representation of an enum map for type `V`, usually an array.
    type Array: Array<V>;
    /// Takes an usize, and returns an element matching `to_usize` function.
    fn from_usize(value: usize) -> Self;
    /// Returns an unique identifier for a value within range of `0..Array::LENGTH`.
    fn to_usize(self) -> usize;
    /// Creates an array using a function called for each argument.
    fn from_function<F: FnMut(Self) -> V>(f: F) -> Self::Array;
}

pub trait Array<V> {
    const LENGTH: usize;
    fn slice(&self) -> &[V];
    fn slice_mut(&mut self) -> &mut [V];
}

impl<V, const N: usize> Array<V> for [V; N] {
    const LENGTH: usize = N;
    fn slice(&self) -> &[V] {
        self
    }
    fn slice_mut(&mut self) -> &mut [V] {
        self
    }
}

impl<T> Enum<T> for bool {
    type Array = [T; 2];
    #[inline]
    fn from_usize(value: usize) -> Self {
        match value {
            0 => false,
            1 => true,
            _ => unreachable!(),
        }
    }
    #[inline]
    fn to_usize(self) -> usize {
        self as usize
    }
    #[inline]
    fn from_function<F: FnMut(Self) -> T>(mut f: F) -> [T; 2] {
        [f(false), f(true)]
    }
}

impl<T> Enum<T> for u8 {
    type Array = [T; 256];
    #[inline]
    fn from_usize(value: usize) -> Self {
        value as u8
    }
    #[inline]
    fn to_usize(self) -> usize {
        self as usize
    }
    #[inline]
    fn from_function<F: FnMut(Self) -> T>(mut f: F) -> [T; 256] {
        array![|i| f(i as u8); 256]
    }
}

impl<T> Enum<T> for Infallible {
    type Array = [T; 0];
    #[inline]
    fn from_usize(_: usize) -> Self {
        unreachable!();
    }
    #[inline]
    fn to_usize(self) -> usize {
        match self {}
    }
    #[inline]
    fn from_function<F: FnMut(Self) -> T>(_: F) -> [T; 0] {
        []
    }
}
