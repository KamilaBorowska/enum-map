/// Internal enum mapping type
///
/// This trait is internally used by `#[derive(EnumMap)]`. Despite it being
/// a public trait, it's not intended to be public. `Internal<T>` is
/// implemented by any enum type where V is a generic type representing a
/// value. The purpose of this generic type is to allow providing a value
/// type for arrays, as Rust currently does not support higher kinded types.
/// Users aren't expected to use this trait directly, but rather to use
/// `EnumMap<K, V>` which uses this internally.
///
/// This trait is also implemented by `bool`, `u8` and `Option` type. While `u8` is
/// strictly speaking not an actual enum, there are good reasons to consider
/// it like one, as array of `u8` keys is a relatively common pattern.
pub trait Internal<V>: Sized {
    /// Representation of an enum map for type `V`, usually an array.
    type Array;
    #[doc(hidden)]
    fn slice(array: &Self::Array) -> &[V];
    #[doc(hidden)]
    fn slice_mut(array: &mut Self::Array) -> &mut [V];
    #[doc(hidden)]
    fn from_usize(value: usize) -> Self;
    #[doc(hidden)]
    fn to_usize(self) -> usize;
    #[doc(hidden)]
    fn from_function<F: FnMut(Self) -> V>(f: F) -> Self::Array;
}

impl<T> Internal<T> for bool {
    type Array = [T; 2];
    fn slice(array: &[T; 2]) -> &[T] {
        array
    }
    fn slice_mut(array: &mut [T; 2]) -> &mut [T] {
        array
    }
    fn from_usize(value: usize) -> Self {
        match value {
            0 => false,
            1 => true,
            _ => unreachable!(),
        }
    }
    fn to_usize(self) -> usize {
        self as usize
    }
    fn from_function<F: FnMut(Self) -> T>(mut f: F) -> [T; 2] {
        [f(false), f(true)]
    }
}

impl<T> Internal<T> for u8 {
    type Array = [T; 256];
    fn slice(array: &[T; 256]) -> &[T] {
        array
    }
    fn slice_mut(array: &mut [T; 256]) -> &mut [T] {
        array
    }
    fn from_usize(value: usize) -> Self {
        value as u8
    }
    fn to_usize(self) -> usize {
        self as usize
    }
    fn from_function<F: FnMut(Self) -> T>(mut f: F) -> [T; 256] {
        array![|i| f(i as u8); 256]
    }
}

// C representation is needed to ensure Rust compiler won't reorder those 2 fields.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OptionInternal<T, U> {
    none: T,
    some: U,
}
