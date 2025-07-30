pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let max_value = nums.iter().copied().max().expect("no value in input");

    let mut count = 0;
    let mut result = 0;

    for v in nums {
        if v == max_value {
            count += 1;
            result = result.max(count);
        } else {
            count = 0;
            continue;
        }
    }
    result
}
