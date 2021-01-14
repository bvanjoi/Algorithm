pub fn max_subarray_dp(arr: Vec<i32>) -> i32 {
    if arr.len() == 0 {
        panic!("The array's length must be greater 0");
    }
    let mut max_sum = arr[0];
    let mut current_sum = arr[0];
    for value in arr {
        current_sum = value.max(current_sum + value);
        max_sum = max_sum.max(current_sum);
    }
    max_sum
}

pub fn max_subarray_divide_and_conquer(arr: Vec<i32>) -> i32 {
    fn find_max_crossing_subarray(arr: &mut Vec<i32>, left: i32, mid: i32, right: i32) -> i32 {
        let mut left_max_sum = i32::min_value();
        let mut sum = 0;
        for i in (left..=mid).rev() {
            sum += arr[i as usize];
            if sum > left_max_sum {
                left_max_sum = sum;
            }
        }
        let mut right_max_sum = i32::min_value();
        let mut sum = 0;
        for i in mid + 1..=right {
            sum += arr[i as usize];
            if sum > right_max_sum {
                right_max_sum = sum;
            }
        }
        return left_max_sum + right_max_sum;
    }
    fn recursive(arr: &mut Vec<i32>, left: i32, right: i32) -> i32 {
        if left == right {
            // 递归的基准情况
            return arr[left as usize];
        }
        let mid = left + (right - left) / 2;
        let max_left_sum = recursive(arr, left, mid);
        let max_right_sum = recursive(arr, mid + 1, right);
        let max_crossing_subarray = find_max_crossing_subarray(arr, left, mid, right);
        return i32::max(max_left_sum, max_right_sum).max(max_crossing_subarray);
    }
    let len = arr.len() as i32;
    let arr = &mut arr.clone();
    recursive(arr, 0, len - 1)
}

pub fn max_subarray_brute_force(arr: Vec<i32>) -> i32 {
    if arr.len() == 0 {
        panic!("The array's length must be greater 0");
    }
    let mut max_sum = arr[0];
    for i in 0..arr.len() {
        let mut now_sum =0;
        for j in i..arr.len() {
            now_sum += arr[j];
            max_sum = i32::max(max_sum, now_sum);
        }
    }
    max_sum
}
