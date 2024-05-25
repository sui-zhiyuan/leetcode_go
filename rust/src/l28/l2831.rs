use std::collections::HashMap;

pub fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
    let mut pos = HashMap::<i32, Vec<usize>>::new();
    let k = k as usize;
    for (i, v) in nums.iter().enumerate() {
        pos.entry(*v).or_default().push(i);
    }

    let mut result = 1;
    for v in pos.values() {
        let (mut start, mut end) = (0, 0);
        while end < v.len() - 1 {
            end += 1;

            loop {
                let count = end + 1 - start;
                if v[end] + 1 - v[start] - count <= k {
                    result = result.max(count);
                    break;
                }

                start += 1;
            }
        }
    }
    result as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![1, 2, 2, 3, 1, 1, 1, 2, 3, 2, 2];
        let k = 2;
        assert_eq!(longest_equal_subarray(nums, k), 3);
    }

    #[test]
    fn test1() {
        let nums = vec![1, 3, 2, 3, 1, 3];
        let k = 3;
        assert_eq!(longest_equal_subarray(nums, k), 3);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 1, 2, 2, 1, 1];
        let k = 2;
        assert_eq!(longest_equal_subarray(nums, k), 4);
    }
}
