#![no_std]

#[macro_use]
extern crate enum_map;
#[macro_use]
extern crate enum_map_derive;

#[derive(Debug, EnumMap, PartialEq)]
enum Example {
    A,
    B,
}

#[test]
fn iterator_len() {
    assert_eq!(enum_map! { Example::A | Example::B => 0 }.iter().len(), 2);
}

#[test]
fn iterator_next_back() {
    assert_eq!(enum_map! { Example::A => 1, Example:: B => 2 }
                   .iter()
                   .next_back(),
               Some((Example::B, &2)));
}
