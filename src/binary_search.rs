type Index = usize;

/// # Binary Search
///
/// If a subarray had already sorted, the searching algorithm
/// can check the midpoint of the subarray against `v` and eliminate
/// half of the subarray from further consideration.
///
pub fn binary_search<T: Ord>(sorted_arr: &Vec<T>, ele: &T) -> Option<Index> {
    let mut left = 0;
    let mut right = sorted_arr.len();
    while left < right {
        use std::cmp::Ordering::*;
        let mid = left + (right - left) / 2;
        let cmp = sorted_arr[mid].cmp(ele);
        dbg!(mid);
        match cmp {
            Less => {
                left = mid + 1;
            }
            Equal => {
                return Some(mid);
            }
            Greater => {
                right = mid;
            }
        }
    }
    None
}

#[test]
pub fn binary_search_test() {
    use pretty_assertions::assert_eq;
    assert_eq!(binary_search(&vec![0, 1, 3, 4, 5], &0), Some(0));
    assert_eq!(binary_search(&vec![0, 1, 3, 4, 5], &1), Some(1));
    assert_eq!(binary_search(&vec![0, 1, 3, 4, 5], &2), None);
    assert_eq!(binary_search(&vec![0, 1, 3, 4, 5], &3), Some(2));
    assert_eq!(binary_search(&vec![0, 1, 3, 4, 5], &4), Some(3));
    assert_eq!(binary_search(&vec![0, 1, 3, 4, 5], &5), Some(4));
    assert_eq!(binary_search(&vec![0, 1, 3, 4, 5], &-1), None);
    assert_eq!(binary_search(&vec![0, 1, 3, 4, 5], &6), None);
}
