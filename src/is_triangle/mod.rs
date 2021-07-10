pub fn is_triangle(a: i32, b: i32, c: i32) -> bool {
    a > 0 && b > 0 && c > 0 && a + b > c && (a - b).abs() < c
}

#[test]
fn test_is_triangle() {
    assert_eq!(is_triangle(2, 3, 4), true);
    assert_eq!(is_triangle(2, 2, 3), true);
    assert_eq!(is_triangle(2, 2, 4), false);
    assert_eq!(is_triangle(0, 2, 4), false);
}
