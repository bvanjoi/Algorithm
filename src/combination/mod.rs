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
