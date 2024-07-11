// cspell:ignore incremovable,subarray

pub fn incremovable_subarray_count(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut prefix = 0;
    while prefix + 1 < n && nums[prefix + 1] > nums[prefix] {
        prefix += 1;
    }
    let mut appendix = n - 1;
    while appendix > 0 && nums[appendix - 1] < nums[appendix] {
        appendix -= 1;
    }

    let mut end = appendix;
    let mut result: i64 = 0;
    for start in 0..=(prefix + 1).min(n - 1) {
        let pre_start_value = if start == 0 {
            i32::MIN
        } else {
            nums[start - 1]
        };

        while end <= start || (end < n && nums[end] <= pre_start_value) {
            end += 1;
        }

        result += (n - end + 1) as i64;
    }

    result
}
