use std::cmp;

impl Solution {
    pub fn min_skips(dist: Vec<i32>, speed: i32, hours_before: i32) -> i32 {
        let require = hours_before * speed;
        let mut curr = &mut vec![0; dist.len()];
        let mut time = 0;
        for (i, &v) in dist.iter().enumerate() {
            curr[i] = time + v;
            time = cell(curr[i], speed)
        }
        if *curr.last().unwrap() <= require {
            return 0;
        }
        let mut next = &mut vec![0; dist.len()];
        // dbg!(require , &curr);

        for i in 1..dist.len() {
            let mut prev = 0;
            for (j, v) in next.iter_mut().enumerate() {
                prev = if j == 0 {
                    dist[0]
                } else {
                    cmp::min(cell(prev, speed) + dist[j], curr[j - 1] + dist[j])
                };
                *v = prev;
            }
            (curr, next) = (next, curr);
            // dbg!(&curr);
            if *curr.last().unwrap() <= require {
                return i as i32;
            }
        }
        -1
    }
}

fn cell(v: i32, speed: i32) -> i32 {
    (v + speed - 1) / speed * speed
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let dist = vec![1, 3, 2];
        let speed = 4;
        let hours_before = 2;
        let res = 1;
        assert_eq!(Solution::min_skips(dist, speed, hours_before), res);
    }

    #[test]
    fn test2() {
        let dist = vec![7, 3, 5, 5];
        let speed = 2;
        let hours_before = 10;
        let res = 2;
        assert_eq!(Solution::min_skips(dist, speed, hours_before), res);
    }

    #[test]
    fn test3() {
        let dist = vec![7, 3, 5, 5];
        let speed = 1;
        let hours_before = 10;
        let res = -1;
        assert_eq!(Solution::min_skips(dist, speed, hours_before), res);
    }

    #[test]
    fn test4() {
        let dist = vec![2, 1, 5, 4, 4, 3, 2, 9, 2, 10];
        let speed = 6;
        let hours_before = 7;
        let res = 7;
        assert_eq!(Solution::min_skips(dist, speed, hours_before), res);
    }
}
