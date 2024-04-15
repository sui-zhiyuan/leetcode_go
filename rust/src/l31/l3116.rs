impl Solution {
    pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
        let total = 1 << coins.len();
        let mut lma = Vec::with_capacity(total);
        lma.push(1);
        for i in 1..total {
            if i.count_ones() == 1 {
                lma.push(coins[i.trailing_zeros() as usize] as i64);
                continue;
            }
            let last = 1 << (i.trailing_zeros() as usize);
            lma.push(lcm(lma[i - last], lma[last]))
        }

        // for v in lma.iter() {
        //     println!("{:?}", v);
        // }

        let &max = lma.last().unwrap();
        let get_count = |max| {
            let mut count = 0;
            for (i, &v) in lma.iter().enumerate().skip(1) {
                let switch = match i.count_ones() % 2 {
                    0 => -1,
                    1 => 1,
                    _ => unreachable!(),
                };
                count += switch * (max / v);
            }
            count
        };
        let count = get_count(max);
        let t = (k as i64 / count) as i32;
        let k = (k as i64 % count) as i32;
        //dbg!(count, k);
        let (mut left, mut right) = (0, max);
        while left < right - 1 {
            let mid = left + (right - left) / 2 - 1;
            // let mk = get_count(mid);
            //dbg!((left, right, mk));
            if get_count(mid) < k as i64 {
                left = mid + 1;
            } else {
                right = mid + 1;
            }
        }

        (t as i64) * max + left
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
        assert_eq!(Solution::find_kth_smallest(vec![5, 2], 7), 12);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::find_kth_smallest(vec![5, 7], 500), 1590);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::find_kth_smallest(vec![1, 10, 100], 1000), 1000);
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::find_kth_smallest(
                vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 25, 20, 18],
                1000000000
            ),
            1195583314
        );
    }
}
