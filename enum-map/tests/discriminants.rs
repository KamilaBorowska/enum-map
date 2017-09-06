#![no_std]

#[macro_use]
extern crate enum_map;

use enum_map::EnumMap;

#[derive(Debug, EnumMap, PartialEq)]
enum Discriminants {
    A = 2,
    B = 0,
    C = 1,
}

#[test]
fn discriminants() {
    let mut map = EnumMap::new();
    map[Discriminants::A] = 3;
    map[Discriminants::B] = 2;
    map[Discriminants::C] = 1;
    let mut pairs = map.into_iter();
    assert_eq!(pairs.next(), Some((Discriminants::A, &3)));
    assert_eq!(pairs.next(), Some((Discriminants::B, &2)));
    assert_eq!(pairs.next(), Some((Discriminants::C, &1)));
    assert_eq!(pairs.next(), None);
}
