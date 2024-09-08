pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
        if nums[left].abs() < nums[right].abs() {
            result.push(nums[right].pow(2));
            right -= 1;
        } else {
            result.push(nums[left].pow(2));
            left += 1;
        }
    }
    result.reverse();
    result
}