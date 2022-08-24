extern crate intro;

use intro::cosas::sort;

#[test]
fn sort_short() {
    let numbers = vec![4, 5, 2, 7];
    let sorted = sort(&numbers);
    assert_eq!(sorted, vec![2, 4, 5, 7]);
}