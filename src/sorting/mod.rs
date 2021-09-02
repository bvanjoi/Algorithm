pub mod bubble_sort;
pub mod heap_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod selection_sort;

use self::{
    bubble_sort::bubble_sort, heap_sort::heap_sort, insertion_sort::insertion_sort,
    insertion_sort::insertion_sort_recursive, merge_sort::merge_sort, quick_sort::quick_sort,
    selection_sort::selection_sort,
};

fn test_sorts(arr: Vec<i32>) {
    let mut compare_arr = arr.clone();
    compare_arr.sort();

    let arr_after_merge_sort = merge_sort(arr.clone());
    let arr_after_insertion_sort = insertion_sort(arr.clone());
    let arr_after_insertion_sort_recursive = insertion_sort_recursive(arr.clone());
    let arr_after_selection_sort = selection_sort(arr.clone());
    let arr_after_bubble_sort = bubble_sort(arr.clone());
    let arr_after_heap_sort = heap_sort(arr.clone());
    let arr_after_quick_sort = quick_sort(arr.clone());

    assert_eq!(
        compare_arr, arr_after_insertion_sort,
        "Failed in insertion_sort by using {:?} and {:?}",
        compare_arr, arr_after_insertion_sort
    );

    assert_eq!(
        compare_arr, arr_after_insertion_sort_recursive,
        "Failed in insertion_sort_recursive by using {:?} and {:?}",
        compare_arr, arr_after_insertion_sort_recursive
    );

    assert_eq!(
        compare_arr, arr_after_selection_sort,
        "Failed in selection_sort by using {:?} and {:?}",
        compare_arr, arr_after_selection_sort
    );

    assert_eq!(
        compare_arr, arr_after_merge_sort,
        "Failed in merge_sort by using {:?} and {:?}",
        compare_arr, arr_after_merge_sort
    );

    assert_eq!(
        compare_arr, arr_after_bubble_sort,
        "Failed in bubble_sort by using {:?} and {:?}",
        compare_arr, arr_after_bubble_sort
    );

    assert_eq!(
        compare_arr, arr_after_heap_sort,
        "Failed in heap_sort by using {:?} and {:?}",
        compare_arr, arr_after_heap_sort
    );

    assert_eq!(
        compare_arr, arr_after_quick_sort,
        "Failed in quick_sort by using {:?} and {:?}",
        compare_arr, arr_after_quick_sort
    );
}

#[test]
fn ordered() {
    test_sorts(vec![1]);
    test_sorts(vec![1, 2]);
    test_sorts(vec![1, 2, 3, 4, 5]);
    test_sorts(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn reversed() {
    test_sorts(vec![5, 1]);
    test_sorts(vec![5, 4, 3, 2, 1]);

    let mut arr: Vec<i32> = Vec::new();
    for i in 0..1000 {
        arr.push(1000 - i);
    }

    test_sorts(arr);
}

#[test]
fn normal() {
    test_sorts(vec![2, 4, 5, 7, 1, 2, 3, 6]);
    test_sorts(vec![31, 41, 59, 26, 41, 58]);
    test_sorts(vec![3, 4, 2, 1, 7, 5, 8, 9, 0, 6]);
    test_sorts(vec![3, 41, 52, 26, 38, 57, 9, 49]);
    test_sorts(vec![
        2, 123, 4, 21, 41, 5, 423, 43, 5643, 6, 5347, 45, 7, 6454, 3, 21, 14, 124, 12, 4, 124,
        2315, 32,
    ]);
}
