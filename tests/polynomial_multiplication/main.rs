use algorithm::horner_method::horner_method;

#[test]
fn test_inversion() {
    assert_eq!(horner_method(vec![1, 2, 3, 4], 2), 49);
    assert_eq!(horner_method(vec![-1, 2, -6, 2], 3), 5);
}
