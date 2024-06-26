use std::ops::Range;

pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![-1; nums.len()];

    let mut monotonic_stack = Vec::new();

    for (i, &v) in nums.iter().enumerate().rev().cycle().take(nums.len() * 2) {
        if monotonic_stack.is_empty() {
            monotonic_stack.push(v);
            continue;
        }

        let n = binary_search(0..monotonic_stack.len(), |i| monotonic_stack[i] <= v);
        if n > 0 {
            result[i] = monotonic_stack[n - 1];
        }

        if n < monotonic_stack.len() {
            monotonic_stack.truncate(n);
        }
        monotonic_stack.push(v);
    }

    result
}

fn binary_search(r: Range<usize>, mut f: impl FnMut(usize) -> bool) -> usize {
    let mut left = r.start;
    let mut right = r.end;

    while left < right - 1 {
        let mid = left + (right - left) / 2;
        if f(mid) {
            right = mid;
        } else {
            left = mid;
        }
    }

    if f(left) {
        left
    } else {
        right
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1,2,1];
        let res = vec![2,-1,2];
        assert_eq!(next_greater_elements(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![1,2,3,4,3];
        let res = vec![2,3,4,-1,4];
        assert_eq!(next_greater_elements(nums), res);
    }
}