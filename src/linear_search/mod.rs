pub fn linear_search(arr: Vec<i32>, v: i32) -> i32 {
    let index = -1;
    for i in 0..arr.len() {
        if arr[i] == v {
            return i as i32;
        }
    }
    return index;
}
