// Creating some basic integration tests
// using chapter 11 of 'The Rust Programming Language'

use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}