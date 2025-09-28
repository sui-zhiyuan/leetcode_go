use std::cmp::Reverse;

pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable_by_key(|&x| Reverse(x));

    for (i, &a) in nums.iter().enumerate() {
        if i + 2 >= nums.len() {
            break;
        }

        let b = nums[i + 1];
        let c = nums[i + 2];

        if b + c > a {
            return a + b + c;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameter_tests;

    struct Case {
        input: Vec<i32>,
        expect: i32,
    }

    fn test(c: Case) {
        assert_eq!(c.expect, largest_perimeter(c.input));
    }

    parameter_tests! {
        test,
        (test1 : Case { input: vec![2, 1, 2], expect: 5 }),
        (test2 : Case { input: vec![1,2,1,10], expect: 0 }),
    }
}
