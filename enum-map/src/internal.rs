use core::convert::Infallible;

/// Enum mapping type
///
/// This trait is internally used by `#[derive(Enum)]`.
///
/// This trait is also implemented by `bool` and `u8`. While `u8` is
/// strictly speaking not an actual enum, there are good reasons to consider
/// it like one, as array of `u8` keys is a relatively common pattern.
pub trait Enum: Sized {
    /// Length of the enum.
    const LENGTH: usize;

    /// Takes an usize, and returns an element matching `into_usize` function.
    fn from_usize(value: usize) -> Self;
    /// Returns an unique identifier for a value within range of `0..Array::LENGTH`.
    fn into_usize(self) -> usize;
}

/// Trait associating
pub trait EnumArray<V>: Enum {
    /// Representation of an enum map for type `V`.
    type Array: Array<V>;
}

/// Array for enum-map storage.
///
/// This trait is inteded for primitive array types (with fixed length).
///
/// # Safety
///
/// The array length needs to match actual storage.
pub unsafe trait Array<V> {
    // This is necessary duplication because the length in Enum trait can be
    // provided by user and may not be trustworthy for unsafe code.
    const LENGTH: usize;

    /// Coerces a reference to the array into a reference to a slice.
    fn slice(&self) -> &[V];

    /// Coerces a mutable reference to the array into a mutable reference to a slice.
    fn slice_mut(&mut self) -> &mut [V];
}

unsafe impl<V, const N: usize> Array<V> for [V; N] {
    const LENGTH: usize = N;
    fn slice(&self) -> &[V] {
        self
    }
    fn slice_mut(&mut self) -> &mut [V] {
        self
    }
}

impl Enum for bool {
    const LENGTH: usize = 2;

    #[inline]
    fn from_usize(value: usize) -> Self {
        match value {
            0 => false,
            1 => true,
            _ => unreachable!(),
        }
    }
    #[inline]
    fn into_usize(self) -> usize {
        usize::from(self)
    }
}

impl<T> EnumArray<T> for bool {
    type Array = [T; Self::LENGTH];
}

impl Enum for u8 {
    const LENGTH: usize = 256;

    #[inline]
    fn from_usize(value: usize) -> Self {
        value.try_into().unwrap()
    }
    #[inline]
    fn into_usize(self) -> usize {
        usize::from(self)
    }
}

impl<T> EnumArray<T> for u8 {
    type Array = [T; Self::LENGTH];
}

impl Enum for Infallible {
    const LENGTH: usize = 0;

    #[inline]
    fn from_usize(_: usize) -> Self {
        unreachable!();
    }
    #[inline]
    fn into_usize(self) -> usize {
        match self {}
    }
}

impl<T> EnumArray<T> for Infallible {
    type Array = [T; Self::LENGTH];
}
