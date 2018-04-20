#[macro_use]
extern crate enum_map;

use enum_map::{EnumMap, IntoIter};

use std::cell::RefCell;

#[derive(Copy, Clone, Debug, EnumMap, PartialEq)]
enum Example {
    A,
    B,
    C,
}

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

#[test]
fn test_debug() {
    let map = enum_map! { false => 3, true => 5 };
    assert_eq!(format!("{:?}", map), "{false: 3, true: 5}");
}

#[test]
fn discriminants() {
    #[derive(Debug, EnumMap, PartialEq)]
    enum Discriminants {
        A = 2000,
        B = 3000,
        C = 1000,
    }
    let mut map = EnumMap::new();
    map[Discriminants::A] = 3;
    map[Discriminants::B] = 2;
    map[Discriminants::C] = 1;
    let mut pairs = map.iter();
    assert_eq!(pairs.next(), Some((Discriminants::A, &3)));
    assert_eq!(pairs.next(), Some((Discriminants::B, &2)));
    assert_eq!(pairs.next(), Some((Discriminants::C, &1)));
    assert_eq!(pairs.next(), None);
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

#[test]
fn huge_enum() {
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

    let map = enum_map! { _ => 2 };
    assert_eq!(map[Example::Xx], 2);
}

#[test]
fn iterator_len() {
    assert_eq!(
        enum_map! { Example::A | Example::B | Example::C => 0 }
            .iter()
            .len(),
        3
    );
}

#[test]
fn iterator_next_back() {
    assert_eq!(
        enum_map! { Example::A => 1, Example::B => 2, Example::C => 3 }
            .iter()
            .next_back(),
        Some((Example::C, &3))
    );
}

struct DropReporter<'a> {
    into: &'a RefCell<Vec<usize>>,
    value: usize,
}

impl<'a> Drop for DropReporter<'a> {
    fn drop(&mut self) {
        self.into.borrow_mut().push(self.value);
    }
}

#[test]
fn into_iter_drop() {
    let dropped = RefCell::new(Vec::new());
    let mut a: IntoIter<Example, _> = enum_map! {
        k => DropReporter {
            into: &dropped,
            value: k as usize,
        },
    }.into_iter();
    assert_eq!(a.next().unwrap().0, Example::A);
    assert_eq!(*dropped.borrow(), &[0]);
    drop(a);
    assert_eq!(*dropped.borrow(), &[0, 1, 2]);
}

#[test]
fn test_u8() {
    let map = enum_map! { b'a' => 4, _ => 0 };
    assert_eq!(map[b'a'], 4);
    assert_eq!(map[b'b'], 0);
}

#[derive(EnumMap)]
enum Void {}

#[test]
fn empty_map() {
    let void: EnumMap<Void, Void> = enum_map!{};
    assert!(void.is_empty());
}

#[test]
#[should_panic]
fn empty_value() {
    let _void: EnumMap<bool, Void> = enum_map! { _ => unreachable!() };
}
