/// # Bubble Sort
///
/// Swapping adjacent elements that are out
/// of order repeatedly.
///
/// ## Disadvantages
///
/// - It's inefficient.
///
pub fn bubble_sort<T: Ord>(arr: Vec<T>) -> Vec<T> {
    let mut arr = arr;
    for i in 0..arr.len() {
        for j in ((i + 1)..arr.len()).rev() {
            if arr[j] < arr[j - 1] {
                arr.swap(j, j - 1)
            }
        }
    }
    arr
}

#[test]
fn bubble_sort_test() {
    let cases = crate::test_helper::sort::cases();
    for case in cases {
        let input = case.get_input().clone();
        let actual = bubble_sort(input);
        pretty_assertions::assert_eq!(&actual, case.get_expected());
    }
}
