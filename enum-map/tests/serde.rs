#![no_std]
#![cfg(feature = "serde")]

#[macro_use]
extern crate enum_map;
#[macro_use]
extern crate serde;
extern crate serde_test;


use serde_test::{Token, assert_tokens};

#[derive(Debug, EnumMap, Deserialize, Serialize)]
enum Example {
    A,
    B,
}

#[test]
fn serialization() {
    let map = enum_map! { Example::A => 5, Example::B => 10 };
    assert_tokens(
        &map,
        &[
            Token::Map { len: Some(2) },
            Token::UnitVariant {
                name: "Example",
                variant: "A",
            },
            Token::I32(5),
            Token::UnitVariant {
                name: "Example",
                variant: "B",
            },
            Token::I32(10),
            Token::MapEnd,
        ],
    );
}
