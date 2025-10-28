pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
    let mut prefix_sum = vec![0i32; nums.len() + 1];
    for (i, v) in nums.iter().enumerate() {
        prefix_sum[i + 1] = prefix_sum[i] + *v;
    }

    let &total_sum = prefix_sum.last().unwrap();
    let mut result = 0;
    for (i, &v) in nums.iter().enumerate() {
        if v != 0 {
            continue;
        }

        let left = prefix_sum[i + 1];
        let right = total_sum - prefix_sum[i];
        if left == right {
            result += 2;
        } else if (left - right).abs() == 1 {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameter_tests;

    struct Case {
        nums: Vec<i32>,
        result: i32,
    }

    fn test(c: Case) {
        assert_eq!(count_valid_selections(c.nums), c.result)
    }

    parameter_tests! {
        test,
        (case1: Case{
            nums: vec![1,0,2,0,3],
            result: 2,
        }),
        (case2: Case{
            nums: vec![2,3,4,0,4,1,0],
            result: 0,
        }),
    }
}
