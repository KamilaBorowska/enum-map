# enum-map

A library providing enum map providing type safe enum array. It is
implemented using regular Rust arrays, so using them is as fast
as using regular Rust arrays.

If you are using Rust 1.35 or older, you may want to use enum-map 0.5 instead,
as enum-map 0.6 requires Rust 1.36.

## Examples

```rust
#[macro_use]
extern crate enum_map;

use enum_map::EnumMap;

#[derive(Debug, Enum)]
enum Example {
    A,
    B,
    C,
}

fn main() {
    let mut map = enum_map! {
        Example::A => 1,
        Example::B => 2,
        Example::C => 3,
    };
    map[Example::C] = 4;

    assert_eq!(map[Example::A], 1);

    for (key, &value) in &map {
        println!("{:?} has {} as value.", key, value);
    }
}
```
