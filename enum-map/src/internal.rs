use core::cmp::Ordering;
use core::convert::Infallible;

/// Enum mapping type.
///
/// This trait is implemented by `#[derive(Enum)]`.
///
/// This trait is also implemented by `bool` and `u8`. While `u8` is
/// strictly speaking not an actual enum, there are good reasons to consider
/// it like one, as array of `u8` keys is a relatively common pattern.
pub trait Enum: Sized {
    /// Representation of an enum.
    ///
    /// For an enum with four elements it looks like this.
    ///
    /// ```
    /// type Array<V> = [V; 4];
    /// ```
    type Array<V>: Array;

    /// Takes an usize, and returns an element matching `into_usize` function.
    fn from_usize(value: usize) -> Self;
    /// Returns an unique identifier for a value within range of `0..Array::LENGTH`.
    fn into_usize(self) -> usize;
}

/// Array for enum-map storage.
///
/// This trait is inteded for primitive array types (with fixed length).
///
/// # Safety
///
/// The array length needs to match actual storage.
pub unsafe trait Array {
    // This is necessary duplication because the length in Enum trait can be
    // provided by user and may not be trustworthy for unsafe code.
    const LENGTH: usize;
}

unsafe impl<V, const N: usize> Array for [V; N] {
    const LENGTH: usize = N;
}

#[doc(hidden)]
#[inline]
pub fn out_of_bounds() -> ! {
    panic!("index out of range for Enum::from_usize");
}

impl Enum for bool {
    type Array<V> = [V; 2];

    #[inline]
    fn from_usize(value: usize) -> Self {
        match value {
            0 => false,
            1 => true,
            _ => out_of_bounds(),
        }
    }
    #[inline]
    fn into_usize(self) -> usize {
        usize::from(self)
    }
}

impl Enum for () {
    type Array<V> = [V; 1];

    #[inline]
    fn from_usize(value: usize) -> Self {
        match value {
            0 => (),
            _ => out_of_bounds(),
        }
    }
    #[inline]
    fn into_usize(self) -> usize {
        0
    }
}

impl Enum for u8 {
    type Array<V> = [V; 256];

    #[inline]
    fn from_usize(value: usize) -> Self {
        value.try_into().unwrap_or_else(|_| out_of_bounds())
    }
    #[inline]
    fn into_usize(self) -> usize {
        usize::from(self)
    }
}

impl Enum for Infallible {
    type Array<V> = [V; 0];

    #[inline]
    fn from_usize(_: usize) -> Self {
        out_of_bounds();
    }
    #[inline]
    fn into_usize(self) -> usize {
        match self {}
    }
}

impl Enum for Ordering {
    type Array<V> = [V; 3];

    #[inline]
    fn from_usize(value: usize) -> Self {
        match value {
            0 => Ordering::Less,
            1 => Ordering::Equal,
            2 => Ordering::Greater,
            _ => out_of_bounds(),
        }
    }
    #[inline]
    fn into_usize(self) -> usize {
        match self {
            Ordering::Less => 0,
            Ordering::Equal => 1,
            Ordering::Greater => 2,
        }
    }
}
