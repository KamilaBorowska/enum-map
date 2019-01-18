//! An enum mapping type.
//!
//! It is implemented using an array type, so using it is as fast as using Rust
//! arrays.
//!
//! # Examples
//!
//! ```
//! use enum_map::{enum_map, Enum, EnumMap};
//!
//! #[derive(Debug, Enum)]
//! enum Example {
//!     A,
//!     B,
//!     C,
//! }
//!
//! fn main() {
//!     let mut map = enum_map! {
//!         Example::A => 1,
//!         Example::B => 2,
//!         Example::C => 3,
//!     };
//!     map[Example::C] = 4;
//!
//!     assert_eq!(map[Example::A], 1);
//!
//!     for (key, &value) in &map {
//!         println!("{:?} has {} as value.", key, value);
//!     }
//! }
//! ```

#![no_std]
#![deny(missing_docs)]

pub use enum_map_derive::Enum;
pub use enum_map_internals::{enum_map, Enum, EnumMap, IntoIter, Iter, IterMut, Values, ValuesMut};
