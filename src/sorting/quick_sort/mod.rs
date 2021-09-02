pub fn quick_sort(arr: Vec<i32>) -> Vec<i32> {
    fn partition(arr: &mut Vec<i32>, p: usize, r: usize) -> usize {
        // 区间内最左侧元素作为比较元素
        let compare_val = arr[p];
        let mut left = p;
        let mut right = r;
        while left < right {
            while arr[right] >= compare_val && left < right {
                right -= 1;
            }
            while arr[left] <= compare_val && left < right {
                left += 1;
            }
            arr.swap(left, right);
        }
        arr.swap(p, left);
        left
    }

    fn sort(arr: &mut Vec<i32>, p: usize, r: usize) {
        if p >= r {
            return;
        }
        let q = partition(arr, p, r);
        sort(arr, p, q);
        sort(arr, q + 1, r);
    }

    let mut arr = arr.clone();
    let len = arr.len();
    sort(&mut arr, 0, len - 1);
    return arr;
}
