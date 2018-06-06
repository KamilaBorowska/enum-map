# Version 0.3.0

## New features

- Implemented compact serde serialization for binary formats like bincode.

- Iterator traits with exception now implement `FusedIterator`.

## Incompatible changes

- Increased required Rust version to 1.26.0.

- Changed `#[derive(EnumMap)]` to `#[derive(Enum)]` to match trait name.

- Renamed `Internal` trait to `Enum`.

- Added new associated constant `POSSIBLE_VALUES` to `Enum` trait,
  representing the number of possible values the type can have. Manual
  implementations are required to provide it.

- Removed `Enum` implementation for `Option<T>`.

- Implemented compact serialization, for formats like `bincode`. This
  makes it impossible to deserialize non-compact representation used by
  enum-map 0.2.0.

- `values` method returns `Values<V>` as opposed to `slice::Iter<V>`.