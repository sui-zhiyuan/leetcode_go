use std::{collections::HashMap, mem};

pub fn special_perm(nums: Vec<i32>) -> i32 {
    let mut curr = &mut HashMap::new();
    let mut next = &mut HashMap::new();
    let n = nums.len();

    const MOD: i32 = 1_000_000_007;

    for (i, &v) in nums.iter().enumerate() {
        curr.insert(
            State {
                last: v,
                used: 1 << i,
            },
            1,
        );
    }

    for _ in 2..=n {
        for (ps, &count) in curr.iter() {
            for (j, &v) in nums.iter().enumerate() {
                if ps.used & (1 << j) != 0 {
                    continue;
                }
                if ps.last % v != 0 && v % ps.last != 0 {
                    continue;
                }

                let ns = State {
                    last: v,
                    used: ps.used | (1 << j),
                };

                let nv = next.entry(ns).or_insert(0);
                *nv += count;
                *nv %= MOD;
            }
        }

        mem::swap(&mut curr, &mut next);
        next.clear();
    }

    curr.values().fold(0, |acc, &v| (acc + v) % MOD)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    last: i32,
    used: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![2, 3, 6];
        let res = 2;
        assert_eq!(special_perm(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 4, 3];
        let res = 2;
        assert_eq!(special_perm(nums), res);
    }
}
