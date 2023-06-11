extern crate test_crate;

use external_crate::my_mod::add;
use test_crate::my_mod::add;

#[test]
fn it_adds_two() {
    assert_eq!(4, add(2, 2));
}
