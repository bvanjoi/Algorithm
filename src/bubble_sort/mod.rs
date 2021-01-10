pub fn bubble_sort(arr: Vec<i32>) -> Vec<i32> {
    let mut arr = arr;
    for i in (0..arr.len()).rev() {
        for j in 1..(i + 1) {
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
            }
        }
    }
    return arr;
}
