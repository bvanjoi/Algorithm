use algorithm::max_subarray::max_subarray_brute_force;
use algorithm::max_subarray::max_subarray_divide_and_conquer;
use algorithm::max_subarray::max_subarray_dp;

#[test]
fn test_max_subarray_dp() {
    assert_eq!(
        max_subarray_dp(vec![-2, -1, -3, -4, -1, -2, -1, -5, -4]),
        -1
    );
    assert_eq!(max_subarray_dp(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    assert_eq!(
        max_subarray_dp(vec![
            13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7
        ]),
        43
    );
}

#[test]
fn test_max_subarray_divide_and_conquer() {
    assert_eq!(
        max_subarray_divide_and_conquer(vec![-2, -1, -3, -4, -1, -2, -1, -5, -4]),
        -1
    );
    assert_eq!(
        max_subarray_divide_and_conquer(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
        6
    );
    assert_eq!(
        max_subarray_divide_and_conquer(vec![
            13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7
        ]),
        43
    );
}

#[test]
fn test_max_subarray_brute_force() {
    assert_eq!(
        max_subarray_brute_force(vec![-2, -1, -3, -4, -1, -2, -1, -5, -4]),
        -1
    );
    assert_eq!(
        max_subarray_brute_force(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
        6
    );
    assert_eq!(
        max_subarray_brute_force(vec![
            13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7
        ]),
        43
    );
}
