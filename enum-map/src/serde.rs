#![cfg(feature = "serde")]

extern crate serde;

use EnumMap;
use Internal;

use self::serde::ser::{Serialize, Serializer, SerializeMap};
use self::serde::de::{self, Deserialize, Deserializer, Error, MapAccess};

use core::fmt;
use core::marker::PhantomData;

/// Requires crate feature `"serde"`
impl<K: Internal<V> + Serialize, V: Serialize> Serialize for EnumMap<K, V> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (key, value) in self {
            map.serialize_entry(&key, value)?;
        }
        map.end()
    }
}

/// Requires crate feature `"serde"`
impl<'de, K, V> Deserialize<'de> for EnumMap<K, V>
where
    K: Internal<V>
        + Internal<Option<V>>
        + Deserialize<'de>,
    V: Deserialize<'de>,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(Visitor(PhantomData))
    }
}

struct Visitor<K, V>(PhantomData<(K, V)>);

impl<'de, K, V> de::Visitor<'de> for Visitor<K, V>
where
    K: Internal<V>
        + Internal<Option<V>>
        + Deserialize<'de>,
    V: Deserialize<'de>,
{
    type Value = EnumMap<K, V>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a map")
    }

    fn visit_map<M: MapAccess<'de>>(self, mut access: M) -> Result<Self::Value, M::Error> {
        let mut entries = enum_map! { _ => None };
        while let Some((key, value)) = access.next_entry()? {
            entries[key] = Some(value);
        }
        for value in entries.values() {
            if value.is_none() {
                return Err(M::Error::custom("key not specified"));
            }
        }
        Ok(enum_map! { key => entries[key].take().unwrap() })
    }
}
