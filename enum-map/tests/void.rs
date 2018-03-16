#![no_std]

#[macro_use]
extern crate enum_map;

use enum_map::EnumMap;

#[derive(EnumMap)]
enum Void {}

#[test]
fn empty_map() {
    let void: EnumMap<Void, Void> = enum_map!{};
    assert!(void.is_empty());
}

#[test]
#[should_panic]
fn empty_value() {
    enum_map! { _ => unreachable!() };
}
