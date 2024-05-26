pub fn maximum_sum_subsequence(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    let mut nums = nums;
    let mut sum = vec![0i64; nums.len()];

    let mut result = 0i64;
    calculate(&mut sum, &nums, nums.len() - 1);
    const MOD: i64 = 1000000007;

    for q in queries {
        let pos = q[0] as usize;
        let x = q[1];
        nums[pos] = x;

        result += calculate(&mut sum, &nums, pos);
        result %= MOD;
    }

    result as i32
}

fn calculate(sum: &mut [i64], nums: &[i32], start: usize) -> i64 {
    for i in (0..=start).rev() {
        let mut v = nums[i].max(0) as i64;
        if i+2 < nums.len() {
            v = v.max(nums[i].max(0) as i64 + sum[i + 2]);
        }
        if i+1 < nums.len() {
            v = v.max(sum[i + 1]);
        }
        sum[i] = v;
    }

    sum[0]
}

#[cfg(test)]
mod tests {
    use crate::common;

    use super::*;

    #[test]
    fn test0() {
        let nums = vec![3,5,9];
        let queries = common::parse_grid::<i32>("[[1,-2],[0,-3]]").unwrap();
        assert_eq!(maximum_sum_subsequence(nums, queries), 21);
    }

    #[test]
    fn test1() {
        let nums = vec![0,-1];
        let queries = common::parse_grid::<i32>("[[0,-5]]").unwrap();
        assert_eq!(maximum_sum_subsequence(nums, queries), 0);
    }
}