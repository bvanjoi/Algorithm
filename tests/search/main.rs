use algorithm::binary_search::binary_search;
use algorithm::linear_search::linear_search;

fn test_linear_search(arr: Vec<i32>, v: i32, right_result: i32) {
    assert_eq!(linear_search(arr, v), right_result);
}

fn test_binary_search(arr: Vec<i32>, v: i32, right_result: i32) {
    assert_eq!(binary_search(arr, v), right_result);
}

#[test]
fn ordered_exist() {
    test_linear_search(vec![0, 1, 2, 3, 4, 5], 0, 0);
    test_linear_search(vec![0, 1, 2, 3, 4, 5], 1, 1);
    test_linear_search(vec![0, 1, 2, 3, 4, 5], 2, 2);
    test_linear_search(vec![0, 1, 2, 3, 4, 5], 3, 3);
    test_linear_search(vec![0, 1, 2, 3, 4, 5], 4, 4);
    test_linear_search(vec![0, 1, 2, 3, 4, 5], 5, 5);

    test_binary_search(vec![0, 1, 2, 3, 4, 5], 0, 0);
    test_binary_search(vec![0, 1, 2, 3, 4, 5], 1, 1);
    test_binary_search(vec![0, 1, 2, 3, 4, 5], 2, 2);
    test_binary_search(vec![0, 1, 2, 3, 4, 5], 3, 3);
    test_binary_search(vec![0, 1, 2, 3, 4, 5], 4, 4);
    test_binary_search(vec![0, 1, 2, 3, 4, 5], 5, 5);
    test_binary_search(vec![10,11,12,16,18,23,29,33,48,54,57,68,77,84,98], 23, 5);
}

#[test]
fn ordered_no_exist() {
    test_linear_search(vec![0, 1, 2, 3, 4, 5], -1, -1);
    test_linear_search(vec![0, 1, 2, 3, 4, 5], 6, -1);

    test_binary_search(vec![0, 1, 2, 3, 4, 5], -1, -1);
    test_binary_search(vec![0, 1, 2, 3, 4, 5], 6, -1);
    test_binary_search(vec![10,11,12,16,18,23,29,33,48,54,57,68,77,84,98], 50, -1);
}

#[test]
fn unordered_exist() {}

#[test]
fn unordered_no_exist() {}
