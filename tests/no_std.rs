#![no_std]

use iter_vals::iter_vals;

#[test]
fn no_std_works() {
    let mut iter = iter_vals!((&1), [..= 1 == 2; &2], [..= 1 == 1; &2], [.. &[3, 4, 5]]);

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), None);
}
