#![cfg(feature = "serde")]

#[macro_use]
extern crate enum_map;
#[macro_use]
extern crate serde;
extern crate serde_json;

use enum_map::EnumMap;

#[derive(Debug, EnumMap, PartialEq, Deserialize, Serialize)]
enum Example {
    A,
    B,
}

const JSON: &str = r#"{"A":5,"B":10}"#;

#[test]
fn json_serialization() {
    let map = enum_map! { Example::A => 5, Example::B => 10 };
    assert_eq!(serde_json::to_string(&map).unwrap(), String::from(JSON));
}

#[test]
fn json_deserialization() {
    let example: EnumMap<Example, i32> = serde_json::from_str(JSON).unwrap();
    assert_eq!(example, enum_map! { Example::A => 5, Example::B => 10 });
}
