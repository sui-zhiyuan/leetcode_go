// cspell:ignore incremovable,subarray

pub fn incremovable_subarray_count(nums: Vec<i32>) -> i32 {
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
    let mut result = 0;
    for start in 0..=(prefix + 1).min(n - 1) {
        let pre_start_value = if start == 0 {
            i32::MIN
        } else {
            nums[start - 1]
        };

        while end <= start || (end < n && nums[end] <= pre_start_value) {
            end += 1;
        }

        result += (n - end + 1) as i32;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(incremovable_subarray_count(vec![1, 2, 3, 4]), 10);
    }

    #[test]
    fn test2() {
        assert_eq!(incremovable_subarray_count(vec![6, 5, 7, 8]), 7);
    }

    #[test]
    fn test3() {
        assert_eq!(incremovable_subarray_count(vec![9, 6, 9]), 4);
    }
}
