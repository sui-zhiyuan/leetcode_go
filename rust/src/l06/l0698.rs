use std::{cmp, collections::HashSet};

pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
    let sum = nums.iter().sum::<i32>();

    if sum % k != 0 {
        return false;
    }
    let mut nums = nums;
    nums.sort_by_key(|v| cmp::Reverse(*v));
    let target = sum / k;

    let mut taken = vec![false; nums.len()];
    dfs(
        &Context { nums, target },
        &mut HashSet::<(u32, i32)>::new(),
        &mut taken,
        0,
        k,
    )
}

struct Context {
    nums: Vec<i32>,
    target: i32,
}

fn dfs(
    ctx: &Context,
    cache: &mut HashSet<(u32, i32)>,
    taken: &mut [bool],
    curr_target: i32,
    k: i32,
) -> bool {
    let value = taken
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &v)| (acc + (v as u32)) << i);
    if cache.contains(&(value, curr_target)) {
        return false;
    }
    if k == 0 {
        return true;
    }

    if curr_target == 0 {
        return dfs(ctx, cache, taken, ctx.target, k - 1);
    }

    for (i, &v) in ctx.nums.iter().enumerate() {
        if v > curr_target || taken[i] {
            continue;
        }

        taken[i] = true;
        if dfs(ctx, cache, taken, curr_target - v, k) {
            return true;
        }
        taken[i] = false;
    }

    cache.insert((value, curr_target));
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![2, 2, 2, 2, 3, 4, 5];
        let k = 4;
        let result = false;
        assert_eq!(can_partition_k_subsets(nums, k), result);
    }

    #[test]
    fn test2() {
        let nums = vec![
            730, 580, 401, 659, 5524, 405, 1601, 3, 383, 4391, 4485, 1024, 1175, 1100, 2299, 3908,
        ];
        let k = 4;
        let result = true;
        assert_eq!(can_partition_k_subsets(nums, k), result);
    }
}
