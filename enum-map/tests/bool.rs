#[macro_use]
extern crate enum_map;

#[test]
fn test_bool() {
    let mut map = enum_map! { false => 24, true => 42 };
    assert_eq!(map[false], 24);
    assert_eq!(map[true], 42);
    map[false] += 1;
    assert_eq!(map[false], 25);
    for (key, item) in &mut map {
        if !key {
            *item += 1;
        }
    }
    assert_eq!(map[false], 26);
    assert_eq!(map[true], 42);
}

#[test]
fn test_option_bool() {
    let mut map = enum_map! { None => 1, Some(false) => 2, Some(true) => 3};
    assert_eq!(map[None], 1);
    assert_eq!(map[Some(false)], 2);
    assert_eq!(map[Some(true)], 3);
    map[None] = 4;
    map[Some(false)] = 5;
    map[Some(true)] = 6;
    assert_eq!(map[None], 4);
    assert_eq!(map[Some(false)], 5);
    assert_eq!(map[Some(true)], 6);
    assert_eq!(map.as_slice(), [4, 5, 6]);
    assert_eq!(
        map.into_iter().collect::<Vec<_>>(),
        [(None, 4), (Some(false), 5), (Some(true), 6)]
    );
}
