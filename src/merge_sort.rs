/// # Merge Sort
///
/// A Sort algorithm closely follows the divide-and-conquer method.
///
/// - Divide: the subarray `A[p: r]` to be sorted into two adjacent
/// subarrays, each of half the size. To do so, compute the midpoint
/// `q` of `A[p: r]`, and divide `A[p: r]` into subarrays `A[p: q]`
/// and `A[q + 1: r]`.
/// - Conquer: sorting `A[p: q]` and `A[q + 1: r]`.
/// - Combine: merging the two sorted subarrays `A[p: q]` and
/// `A[q + 1: r]` back into `A[p: r]`.
///
/// ## Advantages:
///
/// - Stable.
///
/// ## TODO:
///
/// - remove `Clone` in trait `T`.
///
pub fn merge_sort<T: Ord + Clone>(arr: Vec<T>) -> Vec<T> {
    if arr.len() == 0 {
        return arr;
    }
    let mut arr = arr;
    let left = 0;
    let right = arr.len();
    merge_sort_inner(&mut arr, left, right);
    arr
}

fn merge_sort_inner<T: Ord + Clone>(arr: &mut [T], left: usize, right: usize) {
    if left + 1 >= right {
        return;
    }
    let mid = left + (right - left) / 2;
    if left + 1 < right {
        merge_sort_inner(arr, left, mid);
        merge_sort_inner(arr, mid, right);
    }
    // merge
    let mut buf = Vec::new();
    let mut i = left;
    let mut j = mid;

    while i < mid && j < right {
        if arr[i] > arr[j] {
            buf.push(arr[j].clone());
            j += 1;
        } else {
            buf.push(arr[i].clone());
            i += 1;
        }
    }
    while i < mid {
        buf.push(arr[i].clone());
        i += 1;
    }
    while j < right {
        buf.push(arr[j].clone());
        j += 1;
    }

    buf.into_iter().enumerate().for_each(|(index, ele)| {
        arr[left + index] = ele;
    });
}

#[test]
fn merge_sort_test() {
    let cases = crate::test_helper::sort::cases();
    for case in cases {
        let input = case.get_input().clone();
        let actual = merge_sort(input);
        pretty_assertions::assert_eq!(&actual, case.get_expected());
    }
}
