use iter_vals::iter_vals;

#[test]
fn std_works() {
    let iter = iter_vals!((&1), [..= 1 == 2; &2], [..= 1 == 1; &2], [.. &[3, 4, 5]]);
    let vals: Vec<_> = iter.collect();

    assert_eq!(vals, vec![&1, &2, &3, &4, &5]);
}
