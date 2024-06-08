use std::collections::HashSet;

use crate::common::Dim2Array;

pub fn max_operations(nums: Vec<i32>) -> i32 {
    let n = nums.len();

    let sums = vec![
        nums[0] + nums[1],
        nums[0] + nums[nums.len() - 1],
        nums[nums.len() - 1] + nums[nums.len() - 2],
    ]
    .into_iter()
    .collect::<HashSet<i32>>();

    let mut max_count = 1;
    for s in sums {
        let mut cache = Dim2Array::new(n+1, n+1, -1);
        max_count = max_count.max(inner_dfs(&mut cache, &nums, 0, nums.len(), s))
    }

    max_count
}

fn inner_dfs(
    cache: &mut Dim2Array<i32>,
    nums: &[i32],
    start: usize,
    end: usize,
    target: i32,
) -> i32 {
    if end - start < 2 {
        return 0;
    }

    if cache[(start, end)] != -1 {
        return cache[(start, end)];
    }
    let mut max = 0;

    if nums[start] + nums[start+1] == target {
        max = max.max(1 + inner_dfs(cache, nums, 2 + start, end, target));
    }

    if nums[end-1] + nums[end-2] == target {
        max = max.max(1 + inner_dfs(cache, nums, start, end - 2, target));
    }

    if nums[start] + nums[end - 1] == target {
        max = max.max(1 + inner_dfs(cache, nums, start + 1, end - 1, target));
    }

    cache[(start, end)] = max;
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![3, 3, 7, 3, 2, 4, 2, 3];
        assert_eq!(max_operations(nums), 2);
    }
}
