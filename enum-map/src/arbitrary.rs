use crate::internal::Array;
use crate::{enum_map, Enum, EnumMap};
use arbitrary::{Arbitrary, Result, Unstructured};

/// Requires crate feature `"arbitrary"`
impl<'a, K: Enum, V: Arbitrary<'a>> Arbitrary<'a> for EnumMap<K, V> {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<EnumMap<K, V>> {
        Ok(enum_map! {
            _ => Arbitrary::arbitrary(u)?,
        })
    }

    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        let len = K::Array::<V>::LENGTH;
        if len == 0 {
            (0, Some(0))
        } else {
            let (lo, hi) = V::size_hint(depth);
            (
                lo.saturating_mul(len),
                hi.and_then(|hi| hi.checked_mul(len)),
            )
        }
    }
}
