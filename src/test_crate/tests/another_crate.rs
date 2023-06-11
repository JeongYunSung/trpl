extern crate test_crate;

mod common;

use common::sub;
use test_crate::my_mod::add;

#[test]
fn it_adds_two() {
    assert_eq!(4, add(2, 2));
}

#[test]
fn it_subs_two() {
    assert_eq!(0, sub(2, 2));
}