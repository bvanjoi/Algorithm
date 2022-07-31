pub mod bubble_sort;
pub mod counting_sort;
pub mod heap_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod selection_sort;

use self::{
    bubble_sort::bubble_sort, counting_sort::counting_sort, heap_sort::heap_sort,
    merge_sort::merge_sort, quick_sort::quick_sort, selection_sort::selection_sort,
};

fn test_sorts(arr: Vec<i32>) {
    let mut compare_arr = arr.clone();
    compare_arr.sort();

    let arr_after_merge_sort = merge_sort(arr.clone());
    let arr_after_selection_sort = selection_sort(arr.clone());
    let arr_after_bubble_sort = bubble_sort(arr.clone());
    let arr_after_heap_sort = heap_sort(arr.clone());
    let arr_after_quick_sort = quick_sort(arr.clone());
    let arr_after_counting_sort = counting_sort(arr.clone());

    assert_eq!(
        compare_arr, arr_after_counting_sort,
        "Failed in counting_sort by using {:?} and {:?}",
        compare_arr, arr_after_counting_sort
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
