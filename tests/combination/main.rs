use algorithm::combination::combination;

#[test]
fn test_combination() {
    assert_eq!(combination(1, 1), 1);
    assert_eq!(combination(5, 2), 10);
    assert_eq!(combination(6, 3), 20);
    assert_eq!(combination(52, 5), 2598960);
}

#[test]
#[should_panic(expected = "Error! n must greater or equal than k")]
fn test_combination_failed() {
    combination(1, 2);
}
