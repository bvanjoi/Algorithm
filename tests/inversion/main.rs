use algorithm::inversion::inversion_number_merge_sort;

#[test]
fn test_inversion() {
    assert_eq!(inversion_number_merge_sort(vec![]), 0);
    assert_eq!(inversion_number_merge_sort(vec![0]), 0);
    assert_eq!(inversion_number_merge_sort(vec![1, 2]), 0);
    assert_eq!(inversion_number_merge_sort(vec![-1, 0, 1]), 0);
    assert_eq!(inversion_number_merge_sort(vec![2, 1]), 1);
    assert_eq!(inversion_number_merge_sort(vec![3, 2, 1]), 3);
    assert_eq!(inversion_number_merge_sort(vec![3, 5, 4, 8, 2, 6, 9]), 6);
}
