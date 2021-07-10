pub fn inversion_number_merge_sort(arr: Vec<i32>) -> i32 {
    let mut count = 0;

    fn merge_sort_recursive(arr: &mut Vec<i32>, left: usize, right: usize, count: &mut i32) {
        if left + 1 < right {
            let mid = left + (right - left) / 2;
            merge_sort_recursive(arr, left, mid, count);
            merge_sort_recursive(arr, mid, right, count);
            *count += merge(arr, left, mid, right);
        }
    }

    fn merge(arr: &mut Vec<i32>, left: usize, mid: usize, right: usize) -> i32 {
        let mut t: Vec<i32> = Vec::new();
        let mut i = left;
        let mut j = mid;
        let mut c = 0;
        while i < mid && j < right {
            if arr[i] <= arr[j] {
                t.push(arr[i]);
                i = i + 1;
            } else {
                t.push(arr[j]);
                j = j + 1;
                c += (mid - i) as i32;
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
        return c;
    }

    let mut arr: Vec<i32> = arr.clone();
    let left = 0;
    let right = arr.len();
    merge_sort_recursive(&mut arr, left, right, &mut count);
    return count;
}

#[test]
fn test_inversion() {
    assert_eq!(inversion_number_merge_sort(vec![]), 0);
    assert_eq!(inversion_number_merge_sort(vec![0]), 0);
    assert_eq!(inversion_number_merge_sort(vec![1, 2]), 0);
    assert_eq!(inversion_number_merge_sort(vec![-1, 0, 1]), 0);
    assert_eq!(inversion_number_merge_sort(vec![2, 1]), 1);
    assert_eq!(inversion_number_merge_sort(vec![3, 2, 1]), 3);
    assert_eq!(inversion_number_merge_sort(vec![3, 5, 4, 8, 2, 6, 9]), 6);
    assert_eq!(inversion_number_merge_sort(vec![1, 3, 2, 3, 1]), 4);
}
