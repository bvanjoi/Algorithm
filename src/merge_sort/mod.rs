pub fn merge_sort(arr: Vec<i32>) -> Vec<i32> {
    fn merge_sort_recursive(arr: &mut Vec<i32>, left: usize, right: usize) {
        if left + 1 < right {
            let mid = left + (right - left) / 2;
            merge_sort_recursive(arr, left, mid);
            merge_sort_recursive(arr, mid, right);
            merge(arr, left, mid, right);
        }
    }

    fn merge(arr: &mut Vec<i32>, left: usize, mid: usize, right: usize) {
        let mut t: Vec<i32> = Vec::new();
        let mut i = left;
        let mut j = mid;
        while i < mid && j < right {
            if arr[i] < arr[j] {
                t.push(arr[i]);
                i = i + 1;
            } else {
                t.push(arr[j]);
                j = j + 1;
            }
        }
        while i < mid {
            t.push(arr[i]);
            i = i + 1;
        }
        while j < right {
            t.push(arr[j]);
            j = j + 1;
        }
        for k in 0..t.len() {
            arr[k + left] = t[k];
        }
    }

    let mut arr: Vec<i32> = arr.clone();
    let left = 0;
    let right = arr.len();
    merge_sort_recursive(&mut arr, left, right);
    return arr;
}
