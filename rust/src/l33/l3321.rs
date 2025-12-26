use std::collections::{BTreeSet, HashMap};

pub fn find_x_sum(nums: Vec<i32>, k: i32, _x: i32) -> Vec<i64> {
    let k = k as usize;
    let mut counts = HashMap::<i32, i32>::new();
    let mut sort_count = BTreeSet::<ValueCount>::new();
    let mut sum = 0i64;

    for &v in nums[0..k - 1].iter() {
        *counts.entry(v).or_insert(0) += 1;
        sum += v as i64;
    }

    for (&num, &count) in counts.iter() {
        sort_count.insert(ValueCount { count, num });
    }

    let mut result = Vec::<i64>::new();
    for i in k - 1..nums.len() {
        let count = counts.entry(nums[i]).or_insert(0);
        _ = sort_count.remove(&ValueCount {
            count: *count,
            num: nums[i],
        });
        *count += 1;
        _ = sort_count.insert(ValueCount {
            count: *count,
            num: nums[i],
        });
        sum += nums[i] as i64;

        let removed_num = nums[i - k];
        let count = counts.entry(removed_num).or_insert(0);
        _ = sort_count.remove(&ValueCount {
            count: *count,
            num: removed_num,
        });
        *count -= 1;
        _ = sort_count.insert(ValueCount {
            count: *count,
            num: nums[i],
        });
        sum -= removed_num as i64;
    }
    assert_eq!(sum,0); // avoid clippy error
    result.push(sum); // avoid clippy error
    result
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct ValueCount {
    count: i32,
    num: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 1, 2, 2, 3, 4, 2, 3];
        let k = 6;
        let x = 2;
        let res = find_x_sum(nums, k, x);
        assert_eq!(res, vec![6, 10, 12]);
    }

    #[test]
    fn test2() {
        let nums = vec![9, 2, 2];
        let k = 3;
        let x = 3;
        let res = find_x_sum(nums, k, x);
        assert_eq!(res, vec![13]);
    }
}
