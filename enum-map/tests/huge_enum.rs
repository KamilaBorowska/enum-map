#![no_std]

#[macro_use]
extern crate enum_map;

#[derive(EnumMap)]
enum Example {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Aa,
    Bb,
    Cc,
    Dd,
    Ee,
    Ff,
    Gg,
    Hh,
    Ii,
    Jj,
    Kk,
    Ll,
    Mm,
    Nn,
    Oo,
    Pp,
    Qq,
    Rr,
    Ss,
    Tt,
    Uu,
    Vv,
    Ww,
    Xx,
    Yy,
    Zz,
}

#[test]
fn huge_enum() {
    let map = enum_map! { _ => 2 };
    assert_eq!(map[Example::Xx], 2);
}
