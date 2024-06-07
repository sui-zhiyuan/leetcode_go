use std::{
    collections::{BTreeSet, HashMap},
    iter,
};

use crate::common::SegmentTree;

pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
    let dedup_nums = nums
        .iter()
        .copied()
        .chain(iter::once(i32::MAX))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let dedup_n = dedup_nums.len();
    let revert_map = dedup_nums
        .into_iter()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect::<HashMap<_, _>>();

    let mut s1 = SegmentTree::new(vec![0; dedup_n], |a, b| a + b);
    let mut s2 = SegmentTree::new(vec![0; dedup_n], |a, b| a + b);
    let mut arr1 = Vec::new();
    let mut arr2 = Vec::new();

    arr1.push(nums[0]);
    s1.change_value(revert_map[&nums[0]], |v| *v += 1);
    arr2.push(nums[1]);
    s2.change_value(revert_map[&nums[1]], |v| *v += 1);
    for v in nums.into_iter().skip(2) {
        let count1 = s1.range(revert_map[&v] + 1, dedup_n);
        let count2 = s2.range(revert_map[&v] + 1, dedup_n);
        if count1 > count2 {
            arr1.push(v);
            s1.change_value(revert_map[&v], |v| *v += 1);
            continue;
        }

        if count2 > count1 {
            arr2.push(v);
            s2.change_value(revert_map[&v], |v| *v += 1);
            continue;
        }

        if arr1.len() <= arr2.len() {
            arr1.push(v);
            s1.change_value(revert_map[&v], |v| *v += 1);
            continue;
        }

        arr2.push(v);
        s2.change_value(revert_map[&v], |v| *v += 1);
    }

    arr1.append(&mut arr2);
    arr1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![2, 1, 3, 3];
        let except = vec![2, 3, 1, 3];
        assert_eq!(result_array(nums), except);
    }

    #[test]
    fn test2() {
        let nums = vec![5, 14, 3, 1, 2];
        let except = vec![5, 3, 1, 2, 14];
        assert_eq!(result_array(nums), except);
    }

    #[test]
    fn test3() {
        let nums = vec![3, 3, 3, 3];
        let except = vec![3, 3, 3, 3];
        assert_eq!(result_array(nums), except);
    }
}
