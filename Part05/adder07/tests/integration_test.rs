use adder07;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder07::add_two(2));
}
