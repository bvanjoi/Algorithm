pub fn majority_element(nums: Vec<i32>) -> Option<i32> {
    let mut m: i32 = 0;
    let mut c = 0;
    let mut index = 0;
    while index < nums.len() {
        if c == 0 {
            m = nums[index];
            c += 1;
        } else if m == nums[index] {
            c += 1;
        } else {
            c -= 1;
        }
        index += 1;
    }
    let count = nums.iter().filter(|&&x| x == m).count();
    if count > nums.len() / 2 {
        Some(m)
    } else {
        None
    }
}

#[test]
fn test_majority_element() {
    assert_eq!(majority_element(vec![1, 2, 3]), None);
    assert_eq!(majority_element(vec![3, 2]), None);
    assert_eq!(majority_element(vec![3, 2, 3]), Some(3));
    assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), Some(2));
    assert_eq!(majority_element(vec![1, 2, 5, 9, 5, 9, 5, 5, 5]), Some(5));
}
