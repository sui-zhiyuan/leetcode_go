pub fn maximum_beauty(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();

    let mut start = 0;
    let mut result = 0;

    for (end , v) in nums.iter().copied().enumerate() {
        while nums[start] + 2 * k < v {
            start += 1;
        }
        result = result.max(end-start +1);
    }
    result as i32
}