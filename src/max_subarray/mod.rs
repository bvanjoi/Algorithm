pub fn max_subarray(arr: Vec<i32>) -> i32 {
    let mut max_sum = 0;
    let mut current_sum = 0;
    for value in arr {
        current_sum = 0.max(current_sum + value);
        max_sum = max_sum.max(current_sum);
    }
    max_sum
}
