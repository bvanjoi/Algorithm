/// return the value of polynomial multiplication by horner method
///
/// # Arguments
///
/// * `a` - the coefficients of polynomial, the index correspnd to powers
/// * `x` - the value of variable
///
/// # Example
///
/// ```
/// use algorithm::horner_method::horner_method;
/// let a: Vec<i32> = vec![1,2,3,4];
/// let x: i32 = 2;
/// assert_eq!(horner_method(a,x), 49); // the result is ok.
/// ```
pub fn horner_method(a: Vec<i32>, x: i32) -> i128 {
    let mut res: i128 = 0;
    for i in (0..a.len()).rev() {
        res = res * x as i128 + a[i] as i128;
    }
    return res;
}

#[test]
fn test_inversion() {
    assert_eq!(horner_method(vec![1, 2, 3, 4], 2), 49);
    assert_eq!(horner_method(vec![-1, 2, -6, 2], 3), 5);
}
