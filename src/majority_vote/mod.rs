// 多数投票算法
pub fn majority_element(nums: Vec<i32>) -> Option<i32> {
    // 当前元素
    let mut m: i32 = 0;
    // 当前元素出现的次数，若其值为 0, 则将 m 切换为当前迭代的元素。
    let mut c = 0;
    for index in 0..nums.len() {
        if c == 0 {
            m = nums[index];
            c += 1;
        } else if m == nums[index] {
            c += 1;
        } else {
            c -= 1;
        }
    }
    // 判断 m 出现的次数是否大于数组的一半
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
    assert_eq!(majority_element(vec![1, 2, 3, 2, 2, 2, 5, 4, 2]), Some(2));
}
