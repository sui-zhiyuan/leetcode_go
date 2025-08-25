pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut with_0_zero = 0;
    let mut with_1_zero = 0;
    let n = nums.len();

    let mut result: i32 = 0;

    for v in nums {
        if v == 1 {
            with_0_zero += 1;
            with_1_zero += 1;
        } else if v == 0 {
            result = result.max(with_1_zero);
            with_1_zero = with_0_zero;
            with_0_zero = 0;
        } else {
            panic!("invalid value")
        }
    }

    result = result.max(with_0_zero);
    result = result.max(with_1_zero);

    // remove at lest 1 element for input without 0
    if result == (n as i32) {
        result -= 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(2, longest_subarray(vec![1, 1, 1]));
    }
}
