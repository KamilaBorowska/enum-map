use Internal;

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
