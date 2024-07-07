pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
    let mut start = 0;
    let mut prev = nums[0];

    let mut result = 0;
    for (i, &v) in nums.iter().enumerate() {
        if prev == v {
            if i > start {
                let n = (i - start) as i64;
                result += n * (n + 1) / 2;
            }
            prev = v;
            start = i;
            continue;
        }

        prev = v;
    }

    if start < nums.len() {
        let n = (nums.len() - start) as i64;
        result += n * (n + 1) / 2;
    }

    result
}
