pub fn binary_search(arr: Vec<i32>, v: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] == v {
            return mid as i32;
        } else if arr[mid] > v {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    return -1;
}

/// 给定**有序**序列 `arr`, 查找目标值 `target`.
/// - 若存在，则返回包含 `target` 的区间；
/// - 若不存在，则返回 `Option::None`
///
/// # Examples
///
/// ```rust
/// use algorithm::binary_search::binary_search_range;
/// assert_eq!(binary_search_range(vec![5,7,7,8,8,10],8), Some(vec![3,4]));
/// assert_eq!(binary_search_range(vec![5,7,7,8,8,10],6), None);
/// ```
///
pub fn binary_search_range(arr: Vec<i32>, target: i32) -> Option<Vec<usize>> {
    let mut left = 0;
    let mut right = arr.len();

    // 最左侧等于 target 元素的下标
    let mut min_left = None;
    // 最右侧等于 target 元素的下标
    let mut max_right = None;

    while left < right {
        let middle = (right - left) / 2 + left;

        if arr[middle] == target {
            min_left = Some(middle);
            max_right = Some(middle);

            // 寻找最左侧相等的元素
            while left < min_left.unwrap() {
                let m = (min_left.unwrap() - left) / 2 + left;
                if arr[m] == target {
                    min_left = Some(m);
                } else if arr[m] < target {
                    left = m + 1;
                }
            }

            // 寻找最右侧相等的元素
            while right > max_right.unwrap() + 1 {
                let m = (right - max_right.unwrap()) / 2 + max_right.unwrap();
                if arr[m] == target {
                    max_right = Some(m);
                } else if arr[m] > target {
                    right = m;
                }
            }

            break;
        } else if arr[middle] > target {
            right = middle;
        } else {
            left = middle + 1;
        }
    }
    if min_left == None {
        None
    } else {
        Some(vec![min_left.unwrap(), max_right.unwrap()])
    }
}

#[test]
fn test_binary_search_range() {
    assert_eq!(
        binary_search_range(vec![5, 7, 7, 8, 8, 10], 8),
        Some(vec![3, 4])
    );
    assert_eq!(binary_search_range(vec![5, 7, 7, 8, 8, 10], 6), None);
    assert_eq!(binary_search_range(vec![], 0), None);
}
