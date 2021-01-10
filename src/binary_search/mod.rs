pub fn binary_search(arr: Vec<i32>, v: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] == v {
            return mid as i32;
        } else if arr[mid] > v {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    return -1;
}
