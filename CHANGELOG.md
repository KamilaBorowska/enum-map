# Version 0.4.1

## New features

- Default `serde` features are disabled. This allows enabling serde feature when
  compiling without `std`.

# Version 0.4.0

Change of `#[derive(EnumMap)]` to `#[derive(Enum)]` was supposed to appear in 0.3.0,
but it was forgotten about. This release fixes just that.

## Incompatible changes

- Changed `#[derive(EnumMap)]` to `#[derive(Enum)]` to match trait name.

# Version 0.3.1

- Updated README use `#[derive(EnumMap)]` instead of `#[derive(Enum)]`.

# Version 0.3.0

## New features

- Implemented compact serde serialization for binary formats like bincode.

- Iterator traits with exception now implement `FusedIterator`.

## Incompatible changes

- Increased required Rust version to 1.26.0.

- Renamed `Internal` trait to `Enum`.

- Added new associated constant `POSSIBLE_VALUES` to `Enum` trait,
  representing the number of possible values the type can have. Manual
  implementations are required to provide it.

- Removed `Enum` implementation for `Option<T>`.

- Implemented compact serialization, for formats like `bincode`. This
  makes it impossible to deserialize non-compact representation used by
  enum-map 0.2.0.

- `values` method returns `Values<V>` as opposed to `slice::Iter<V>`.
