#[macro_use]
extern crate enum_map;

#[derive(Copy, Clone, Debug, EnumMap, PartialEq)]
enum Example {
    A,
    B,
    C,
}

#[test]
fn extend() {
    let mut map = enum_map! { _ => 0 };
    map.extend(vec![(Example::A, 3)]);
    map.extend(vec![(&Example::B, &4)]);
    assert_eq!(
        map,
        enum_map! { Example:: A => 3, Example:: B => 4, Example::C => 0 }
    );
}
