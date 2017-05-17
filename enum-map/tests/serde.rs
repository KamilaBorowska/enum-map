#![cfg(feature = "serde")]

#[macro_use]
extern crate enum_map;
#[macro_use]
extern crate enum_map_derive;
#[macro_use]
extern crate serde;
extern crate serde_test;


use serde_test::{Token, assert_tokens};

#[derive(Debug, EnumMap, Serialize)]
enum Example {
    A,
    B,
}

#[test]
fn serialization() {
    let map = enum_map! { Example::A => 5, Example::B => 10 };
    assert_tokens(&map,
                  &[Token::Struct {
                        name: "EnumMap",
                        len: 1,
                    },
                    Token::Str("array"),
                    Token::Tuple { len: 2 },
                    Token::I32(5),
                    Token::I32(10),
                    Token::TupleEnd,
                    Token::StructEnd]);
}
