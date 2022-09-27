/// # Selection Sort
///
/// Find the the smallest element of A\[begin_index: n\] and exchanging
/// it with the element in A\[begin_index\], and begin_index is the
/// range 0 to n.
///
/// ## Advantages:
///
/// - in-place.
/// - stable.
///
/// ## Examples
///
/// ```
/// let v = vec![-5, 4, 1, -3, 2];
/// let result = algorithm::selection_sort(v);
/// assert_eq!(result, vec![-5, -3, 1, 2, 4]);
/// ```
///
pub fn selection_sort<T: Ord>(array: Vec<T>) -> Vec<T> {
    let mut array = array;
    for i in 0..array.len() {
        let mut min_index = i;
        for j in i + 1..array.len() {
            if array[j] < array[min_index] {
                min_index = j;
            }
        }
        array.swap(i, min_index);
    }
    array
}

#[test]
fn selection_sort_test() {
    let cases = crate::test_helper::sort::cases();
    for case in cases {
        let input = case.get_input().clone();
        let actual = selection_sort(input);
        assert_eq!(&actual, case.get_expected());
    }
}
