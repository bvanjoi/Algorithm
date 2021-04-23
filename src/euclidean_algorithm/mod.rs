/// # 最大公约数
pub fn gcd(a: i32, b: i32) -> i32 {
  if a < b {
    return gcd(b, a);
  }
  if a % b == 0 {
    b
  } else {
    gcd(b, a % b)
  }
}

# [test]
fn test_gcd() {
  assert_eq!(gcd(4, 3), 1);
  assert_eq!(gcd(6, 9), 3);
  assert_eq!(gcd(10, 15), 5);
  assert_eq!(gcd(252, 105), 21);
  assert_eq!(gcd(462, 1071), 21);
}

/// # 最小公倍数
pub fn lsm(a: i32, b: i32) -> i32 {
  a * b / gcd(a, b)
}
# [test]
fn test_lsm() {
  assert_eq!(lsm(4, 3), 12);
  assert_eq!(lsm(6, 9), 18);
  assert_eq!(lsm(10, 15), 30);
}
