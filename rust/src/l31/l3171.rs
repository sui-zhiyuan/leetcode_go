pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mut state = vec![0 ; nums.len()];
    let mut result = (nums[0] - k).abs();
    for end in 0..nums.len() {
        state[end] = nums[end];
        result = result.min((nums[end] - k).abs());
        for start in (0..end).rev() {
            if state[start] | nums[end] == state[start] {
                break;
            }
            state[start] |= nums[end];
            result = result.min((state[start] - k).abs());
        }
    }
    
    result
}