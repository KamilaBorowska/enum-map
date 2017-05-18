#[macro_use]
extern crate enum_map;

#[test]
fn test_u8() {
    let map = enum_map! { b'a' => b'e', _ => 0 };
    assert_eq!(map[b'a'], b'e');
    assert_eq!(map[b'b'], 0);
}
