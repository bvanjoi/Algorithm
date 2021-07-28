pub fn josephus_permutation(n: i32, m: i32) -> i32 {
    if n == 1 {
        0
    } else {
        (josephus_permutation(n - 1, m) + m) % n
    }
}

#[test]
fn test_josephus_permutation() {
		assert_eq!(josephus_permutation(5, 2), 2);
    assert_eq!(josephus_permutation(5, 3), 3);
    assert_eq!(josephus_permutation(6, 5), 0);
    assert_eq!(josephus_permutation(10, 17), 2);
}
