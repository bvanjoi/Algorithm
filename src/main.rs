fn main() {
    assert_eq!(
        algorithm::max_subarray::max_subarray_divide_and_conquer(vec![
            -2, -1, -3, -4, -1, -2, -1, -5, -4
        ]),
        -1
    );
}
