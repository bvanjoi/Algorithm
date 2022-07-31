/// # Insertion Sort
///
/// Insert the value of the unsorted part to appropriate position of the
/// sorted list one by one to make sure that the list is still ordered
/// after insertion.
///
/// ## Advantages:
///
/// - Insertion sort is an efficient algorithm for sorting a small
/// number of elements.
/// - It's stable.
///
/// ## Examples
///
/// ```
/// let v = vec![-5, 4, 1, -3, 2];
/// let result = algorithm::insertion_sort(v);
/// assert!(result == vec![-5, -3, 1, 2, 4]);
/// ```
///
pub fn insertion_sort<T: Ord>(array: Vec<T>) -> Vec<T> {
    let mut array = array;
    for i in 0..array.len() {
        // I use `swap` rather than insert `array[i]`.
        for j in (0..i).rev() {
            if array[j + 1] > array[j] {
                break;
            }
            array.swap(j + 1, j);
        }
    }
    array
}

#[test]
fn insertion_sort_test() {
    let cases = crate::test_helper::sort::cases();
    for case in cases {
        let input = case.get_input().clone();
        let actual = insertion_sort(input);
        assert_eq!(&actual, case.get_output());
    }
}
