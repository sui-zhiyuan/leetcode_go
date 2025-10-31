pub fn find_maximum_score(nums: Vec<i32>) -> i64 {
    let mut result = 0;
    let mut max = 0;

    for &num in nums[..nums.len() - 1].iter() {
        max = max.max(num);
        result += max as i64;
    }
    result
}
