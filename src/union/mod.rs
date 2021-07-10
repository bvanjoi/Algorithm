pub fn union(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let nums = [nums1.as_slice(), nums2.as_slice()].concat();
    use std::collections::HashSet;
    let set = nums.into_iter().collect::<HashSet<_>>();

    let mut res = set.into_iter().collect::<Vec<_>>();
    // 为了方便 test
    res.sort();
    res
}

#[test]
fn test_union() {
    assert_eq!(union(vec![1, 2, 2, 1], vec![2, 2]), vec![1, 2]);
    assert_eq!(union(vec![4, 9, 5], vec![9, 4, 8, 9, 4]), vec![4, 5, 8, 9]);
}
