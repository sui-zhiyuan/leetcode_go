use std::cmp;

pub fn min_cost_to_equalize_array(nums: Vec<i32>, cost1: i32, cost2: i32) -> i32 {
    let mm = *nums.iter().max().unwrap();
    let n = nums.len() as i64;
    let cost2 = cmp::min(cost1 * 2, cost2);
    let cost1 = cost1 as i64;
    let cost2 = cost2 as i64;

    let (mut max, total) = nums
        .iter()
        .map(|v| mm - v)
        .fold((0i64, 0i64), |(max, total), num| {
            (cmp::max(max, num as i64), total + num as i64)
        });

    let use1 = cmp::min(total / 2, total - max);
    let mut result = use1 as i64 * cost2 as i64;
    result %= MOD_SUM;
    max = if max <= use1 { total % 2 } else { max - use1 };

    if (n - 2) * cost1 >= (n - 1) * cost2 {
        let used = max / (n - 2);
        result += used * ((n - 1) * cost2);
        result %= MOD_SUM;
        max -= used * (n - 2);
    } else {
        result += max * cost1;
        result %= MOD_SUM;
        max = 0;
    }

    let const1 = max * cost1;
    let add1 = max + n;
    let const2 = (add1 / 2) * cost2 + (add1 % 2) * cost1;
    let add2 = max + 2 * n;
    let const3 = (add2 / 2) * cost2 + (add2 % 2) * cost1;
    let const2 = cmp::min(const2, const3);
    result += cmp::min(const1, const2);
    result %= MOD_SUM;

    result as i32
}

const MOD_SUM: i64 = 1000000007;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![4, 1];
        let cost1 = 5;
        let cost2 = 2;
        assert_eq!(min_cost_to_equalize_array(nums, cost1, cost2), 15);
    }

    #[test]
    fn test2() {
        let nums = vec![2, 3, 3, 3, 5];
        let cost1 = 2;
        let cost2 = 1;
        assert_eq!(min_cost_to_equalize_array(nums, cost1, cost2), 6);
    }

    #[test]
    fn test3() {
        let nums = vec![3, 5, 3];
        let cost1 = 1;
        let cost2 = 3;
        assert_eq!(min_cost_to_equalize_array(nums, cost1, cost2), 4);
    }

    #[test]
    fn test4() {
        let nums = vec![1, 14, 14, 15];
        let cost1 = 2;
        let cost2 = 1;
        assert_eq!(min_cost_to_equalize_array(nums, cost1, cost2), 20);
    }

    #[test]
    fn test5() {
        let nums = vec![4, 3, 1, 8];
        let cost1 = 5;
        let cost2 = 1;
        assert_eq!(min_cost_to_equalize_array(nums, cost1, cost2), 8);
    }

    #[test]
    fn test6() {
        let nums = vec![43, 46, 1, 1];
        let cost1 = 41;
        let cost2 = 5;
        assert_eq!(min_cost_to_equalize_array(nums, cost1, cost2), 271);
    }

    #[test]
    fn test7() {
        let nums = vec![55, 52, 29, 11];
        let cost1 = 18;
        let cost2 = 2;
        assert_eq!(min_cost_to_equalize_array(nums, cost1, cost2), 118);
    }

    #[test]
    fn test8() {
        let nums = vec![60, 19, 53, 31, 57];
        let cost1 = 60;
        let cost2 = 2;
        assert_eq!(min_cost_to_equalize_array(nums, cost1, cost2), 90);
    }

    #[test]
    fn test9() {
        let nums = vec![59, 35, 37, 50, 64];
        let cost1 = 14;
        let cost2 = 2;
        assert_eq!(min_cost_to_equalize_array(nums, cost1, cost2), 80);
    }

    #[test]
    fn test10() {
        let mut nums = vec![1000000];
        nums.resize(10_0000, 1);
        let cost1 = 1000000;
        let cost2 = 1000000;
        assert_eq!(min_cost_to_equalize_array(nums, cost1, cost2), 651003857);
    }
}
