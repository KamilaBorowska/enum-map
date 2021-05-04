use crate::{enum_map, Enum, EnumMap};
use arbitrary::{Arbitrary, Result, Unstructured};

impl<'a, K: Enum<V>, V: Arbitrary<'a>> Arbitrary<'a> for EnumMap<K, V> {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<EnumMap<K, V>> {
        Ok(enum_map! {
            _ => Arbitrary::arbitrary(u)?,
        })
    }
}
