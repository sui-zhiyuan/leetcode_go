pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut incr_len = 0;
    let mut result = Vec::new();

    for (i, v) in nums.iter().enumerate() {
        if i == 0 || *v != nums[i - 1] +1 {
            incr_len = 1;
        } else {
            incr_len += 1;
        }

        if i < k - 1 {
            continue;
        }
        if incr_len >= k {
            result.push(*v);
        } else {
            result.push(-1);
        }
    }

    result
}
