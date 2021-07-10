pub fn combination(n: i32, k: i32) -> i32 {
    if n < k {
        panic!("Error! n must greater or equal than k");
    }
    let mut res = 1;
    for i in 1..(k + 1) {
        res = res * (n - k + i) / i;
    }
    return res;
}

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
