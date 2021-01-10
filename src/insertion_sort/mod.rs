pub fn insertion_sort(arr: Vec<i32>) -> Vec<i32> {
    let mut arr = arr;
    for i in 1..arr.len() {
        let (mut j, key) = (i, arr[i]);
        while j != 0 && key < arr[j - 1] {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
    return arr;
}

pub fn insertion_sort_recursive(arr: Vec<i32>) -> Vec<i32> {
    fn insert(arr: &mut Vec<i32>, n: usize) {
        let mut i = n;
        let key = arr[n];
        while key < arr[i - 1] {
            arr[i] = arr[i - 1];
            i -= 1;
            if i == 0 {
                break;
            }
        }
        arr[i] = key;
    }
    fn insert_sort(arr: &mut Vec<i32>, n: usize) {
        if n > 0 {
            insert_sort(arr, n - 1);
            insert(arr, n);
        }
    }
    let mut arr = arr.clone();
    let len = arr.len();
    insert_sort(&mut arr, len - 1);
    arr
}

pub fn insertion_sort_reverse(arr: Vec<i32>) -> Vec<i32> {
    let mut arr = arr;
    for i in 1..arr.len() {
        let (mut j, key) = (i, arr[i]);
        while j != 0 && key > arr[j - 1] {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
    return arr;
}
