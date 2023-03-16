use tdd_learning;

mod common;

fn it_really_adds_two() {
    common::setup();
    assert_eq!(5, tdd_learning::add_two(3));
}