pub fn euler_sieve(n: usize) -> Vec<usize> {
    if n < 2 {
        return vec![];
    }
    let mut r = vec![true; n + 1];

    for i in 2..=n {
        if r[i] == false {
            continue;
        }
        let mut j = 2;
        while i * j < n + 1 {
            r[(i * j)] = false;
            j += 1;
        }
    }
    r.iter()
        .enumerate()
        .filter(|x| x.0 > 1 && *x.1)
        .map(|x| x.0)
        .collect()
}

#[test]
fn test_euler_sieve() {
    assert_eq!(euler_sieve(0), vec![]);
    assert_eq!(euler_sieve(1), vec![]);
    assert_eq!(euler_sieve(2), vec![2]);
    assert_eq!(euler_sieve(10), vec![2, 3, 5, 7]);
    assert_eq!(
        euler_sieve(100),
        vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97
        ]
    )
}
