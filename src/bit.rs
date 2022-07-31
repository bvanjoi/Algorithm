/// # example
///
/// ```
/// use algorithm::bit::only_one_one_bit;
/// assert_eq!(only_one_one_bit(-2147483648), true);
/// assert_eq!(only_one_one_bit(-1), false);
/// assert_eq!(only_one_one_bit(1), true);
/// assert_eq!(only_one_one_bit(0), false);
/// assert_eq!(only_one_one_bit(2), true);
/// assert_eq!(only_one_one_bit(3), false);
/// assert_eq!(only_one_one_bit(4), true);
/// assert_eq!(only_one_one_bit(5), false);
/// assert_eq!(only_one_one_bit(1024), true);
/// assert_eq!(only_one_one_bit(2147483647), false);
/// ```
///
/// |category| time | space|
/// |-|-|-|
/// |complexity| **O(1)** | **O(1)** |
///
pub fn only_one_one_bit(n: i32) -> bool {
    (n != 0) && ((n == i32::MIN) || (n & (n - 1) == 0))
}

/// # example
/// ```
/// use algorithm::bit::is_power_of_two;
/// assert_eq!(is_power_of_two(-2147483648), false);
/// assert_eq!(is_power_of_two(-1), false);
/// assert_eq!(is_power_of_two(0), false);
/// assert_eq!(is_power_of_two(1), true);
/// assert_eq!(is_power_of_two(2), true);
/// assert_eq!(is_power_of_two(3), false);
/// assert_eq!(is_power_of_two(4), true);
/// assert_eq!(is_power_of_two(5), false);
/// assert_eq!(is_power_of_two(1024), true);
/// ```
///
/// |category| time | space|
/// |-|-|-|
/// |complexity| **O(1)** | **O(1)** |
///
pub fn is_power_of_two(n: i32) -> bool {
    n > 0 && only_one_one_bit(n)
}
