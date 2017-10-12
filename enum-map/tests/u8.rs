#![no_std]

#[macro_use]
extern crate enum_map;

#[test]
fn test_u8() {
    let map = enum_map! { b'a' => 4, _ => 0 };
    assert_eq!(map[b'a'], 4);
    assert_eq!(map[b'b'], 0);
}
