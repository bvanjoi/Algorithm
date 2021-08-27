
use rand::Rng;

pub fn fisher_yates_shuffle(nums: Vec<i32>) -> Vec<i32> {
	let mut nums = nums;
	let mut last_index = nums.len() - 1;
	let mut rng = rand::thread_rng();
	while last_index > 0 {
		let random_index = rng.gen_range(0..=last_index);
		nums.swap(random_index, last_index);
		last_index -= 1;
	}
	nums
}