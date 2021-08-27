pub fn dutch_nation_flag(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut left = 0;
    let mut right = nums.len() - 1;
		let mut i = 0;
		while i <= right {
			while i <= right && nums[i] == 2 {
				nums.swap(i, right);
				if right == 0 {
					break;
				}
				right -= 1;
			}

			if nums[i] == 0 {
				nums.swap(i, left);
				left += 1;
			}
			i += 1;
		}
    return nums;
}

#[test]
fn test_dutch_nation_flag() {
    assert_eq!(dutch_nation_flag(vec![1, 0, 2, 1, 1]), vec![0, 1, 1, 1, 2]);
    assert_eq!(dutch_nation_flag(vec![2, 0, 1]), vec![0, 1, 2]);
    assert_eq!(dutch_nation_flag(vec![1, 0, 1]), vec![0, 1, 1]);
    assert_eq!(dutch_nation_flag(vec![1, 2, 1]), vec![1, 1, 2]);
    assert_eq!(dutch_nation_flag(vec![0]), vec![0]);
    assert_eq!(dutch_nation_flag(vec![1]), vec![1]);
    assert_eq!(dutch_nation_flag(vec![2]), vec![2]);
    assert_eq!(dutch_nation_flag(vec![1, 2]), vec![1, 2]);
    assert_eq!(dutch_nation_flag(vec![2, 2]), vec![2, 2]);
    assert_eq!(
        dutch_nation_flag(vec![2, 0, 2, 1, 1, 0]),
        vec![0, 0, 1, 1, 2, 2]
    );
}
