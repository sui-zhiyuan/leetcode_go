use std::collections::BTreeSet;

impl Solution {
    pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
        let lcm_all = coins.iter().fold(1, |acc, &x| lcm(acc, x.into()));

        let mut values = BTreeSet::new();
        for &v in coins.iter() {
            for t in 0..lcm_all / (v as i64) {
                values.insert((v as i64) * t);
            }
        }

        let values = values.into_iter().collect::<Vec<_>>();
        let k = k as usize;
        let times = k / values.len();
        let k = k % values.len();
        values[k] + (times as i64) * lcm_all
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a, b);
    while b > 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(Solution::find_kth_smallest(vec![3, 6, 9], 3), 9);
        assert_eq!(Solution::find_kth_smallest(vec![3, 6, 9], 4), 12);
    }

    #[test]
    fn test_1() {
        assert_eq!(Solution::find_kth_smallest(vec![5,2], 7), 12);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::find_kth_smallest(vec![5, 7], 500), 504);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::find_kth_smallest(vec![1, 10, 100], 1000), 1000);
    }
}
