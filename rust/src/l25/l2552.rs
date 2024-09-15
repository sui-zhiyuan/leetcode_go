pub fn count_quadruplets(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut left_less = vec![vec![0; n + 1]; n];
    let mut right_greater = vec![vec![0; n + 1]; n];
    for i in 1..nums.len() {
        for k in 1..=n {
            if nums[i - 1] < k as i32 {
                left_less[i][k] = left_less[i - 1][k] + 1;
            } else {
                left_less[i][k] = left_less[i - 1][k];
            }
        }
    }

    for i in (0..n - 1).rev() {
        for k in 1..=n {
            if nums[i + 1] > k as i32 {
                right_greater[i][k] = right_greater[i + 1][k] + 1;
            } else {
                right_greater[i][k] = right_greater[i + 1][k];
            }
        }
    }

    let mut result = 0i64;
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] > nums[j] {
                result += left_less[i][nums[j] as usize] as i64 * right_greater[j][nums[i] as usize] as i64;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 3, 2, 4, 5];
        let result = 2;
        assert_eq!(count_quadruplets(nums), result);
    }

    #[test]
    fn test_2() {
        let nums = vec![2, 5, 3, 1, 4];
        let result = 0;
        assert_eq!(count_quadruplets(nums), result);
    }
}
