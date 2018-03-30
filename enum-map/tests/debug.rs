#[macro_use]
extern crate enum_map;

#[test]
fn test_debug() {
    let map = enum_map! { false => 3, true => 5 };
    assert_eq!(format!("{:?}", map), "{false: 3, true: 5}");
}
