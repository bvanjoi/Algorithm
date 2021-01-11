use algorithm::max_subarray::max_subarray;
#[test]
fn test_inversion() {
    assert_eq!(max_subarray(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    assert_eq!(
        max_subarray(vec![
            13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7
        ]),
        43
    );
}
