#[macro_use]
extern crate enum_map;

use enum_map::IntoIter;

use std::cell::RefCell;

#[derive(Debug, EnumMap, PartialEq)]
enum Example {
    A,
    B,
}

#[test]
fn iterator_len() {
    assert_eq!(enum_map! { Example::A | Example::B => 0 }.iter().len(), 2);
}

#[test]
fn iterator_next_back() {
    assert_eq!(
        enum_map! { Example::A => 1, Example:: B => 2 }
            .iter()
            .next_back(),
        Some((Example::B, &2))
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
    assert_eq!(*dropped.borrow(), &[0, 1]);
}
