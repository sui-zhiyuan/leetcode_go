use std::mem;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let (n , neg) = if n<0 {
            (-i64::from(n) , true)
        }else {
            (i64::from(n) , false)
        };
        let mut r = 1f64;
        let start = mem::size_of::<i64>() * 8 - n.leading_zeros() as usize -1;
        let mut p = 1i64 << start;
        while p > 0 {
            r *= r;
            if n & p == p {
                r *= x;
            }
            p >>= 1;
        }
        if neg {
            1.0 / r
        } else {
            r
        }
    }
}

pub struct Solution();

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_approx_eq {
        ($a:expr, $b:expr) => {{
            let (a, b) = (&$a, &$b);
            assert!((*a - *b).abs() < 1.0e-6, "{} is not approximately equal to {}", *a, *b);
        }};
    }

    #[test]
    fn test() {
        assert_approx_eq!(Solution::my_pow(2.0, 10), 1024.0);
        assert_approx_eq!(Solution::my_pow(2.1, 3), 9.261);
        assert_approx_eq!(Solution::my_pow(2.0, -2), 0.25);
        assert_approx_eq!(Solution::my_pow(2.0, -2147483648), 0.0);
    }
}
