# Version 1.0.0

## New features

- It's now possible to use `return` and `?` within `macro_rules!` macro.

- `Enum` trait is much simpler having two methods only.

## Other changes

- Removed previously deprecated features.

- Renamed `to_usize` to `into_usize` matching the naming convention
  used in Rust programming language.

# Version 0.6.5

## Other changes

- Deprecated `EnumMap::is_empty` and `EnumMap::new`. `EnumMap::new` usages
  can be replaced with `EnumMap::default`.

# Version 0.6.4

## Other changes

- Deprecated `EnumMap::as_ptr` and `EnumMap::as_mut_ptr`.

# Version 0.6.3

## New features

- `Iter` and `Values` now implements `Clone` (added by @amanieu).

# Version 0.6.2.

## New features

- Added `EnumMap#clear` method (added by @Riey, thanks :)).

# Version 0.6.0

## Incompatible changes

- Now requires Rust 1.36.

# Version 0.5.0

- Fixed the issue where an aliasing `From` trait implementation caused
  compilation errors with `enum_map!` macro.

## Incompatible changes

- Now requires Rust 1.31.

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
