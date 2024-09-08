pub fn count_ways(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();

    let mut result = 0;
    for i in 0..=nums.len() {
        if (i == 0 || nums[i - 1] < i as i32) && (i >= nums.len() || nums[i] > i as i32) {
            result += 1;
        }
    }

    result
}
