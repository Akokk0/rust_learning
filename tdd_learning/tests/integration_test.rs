use tdd_learning;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, tdd_learning::add_two(2));
}