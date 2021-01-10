pub fn selection_sort(arr: Vec<i32>) -> Vec<i32> {
    let mut arr = arr;
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
    return arr;
}
