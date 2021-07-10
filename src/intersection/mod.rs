pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    use std::collections::HashSet;

    let set1 = nums1.into_iter().collect::<HashSet<_>>();
    let set2 = nums2.into_iter().collect::<HashSet<_>>();

    let mut res = vec![];

    for i in set1.iter() {
        if let Some(&val) = set2.get(i) {
            res.push(val.clone());
        }
    }
    // 为了方便 test
    res.sort();
    res
}

#[test]
fn test_intersection() {
    assert_eq!(intersection(vec![1, 2, 2, 1], vec![2, 2]), vec![2]);
    assert_eq!(intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![4, 9]);
}
