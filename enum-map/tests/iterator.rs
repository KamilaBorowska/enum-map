#[macro_use]
extern crate enum_map;
#[macro_use]
extern crate enum_map_derive;

#[derive(EnumMap)]
enum Example {
    A,
    B,
}

#[test]
fn iterator_len() {
    assert_eq!(enum_map! { Example::A | Example::B => 0 }.iter().len(), 2);
}
