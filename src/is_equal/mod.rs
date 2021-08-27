pub fn is_equal(x: i32, y: i32) -> bool {
    (x ^ y) == 0
}

#[test]
fn test_is_equal() {
    assert_eq!(is_equal(1, 1), true);
    assert_eq!(is_equal(-1, -1), true);
    assert_eq!(is_equal(-1, 1), false);
    assert_eq!(is_equal(0, 1), false);
}
