pub fn is_power_of_two(n: i32) -> bool {
    // n > 0 是因为 2 的任何幂都是正数
    // n & (n - 1)  == 0  用以判断二进制中是否只有一个 1.
    n > 0 && n & (n - 1) == 0
}

#[test]
fn test_is_power_of_two() {
    assert_eq!(is_power_of_two(-1), false);
    assert_eq!(is_power_of_two(0), false);
    assert_eq!(is_power_of_two(1), true);
    assert_eq!(is_power_of_two(2), true);
    assert_eq!(is_power_of_two(3), false);
    assert_eq!(is_power_of_two(4), true);
    assert_eq!(is_power_of_two(5), false);
    assert_eq!(is_power_of_two(1024), true);
}
