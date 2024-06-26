use std::collections::HashMap;

pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut monotonic_stack = Vec::new();

    let mut result = vec![-1; nums1.len()];
    let v_map = nums1
        .into_iter()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect::<HashMap<i32, usize>>();

    for &v in nums2.iter().rev() {
        let (index, pre_v) = monotonic_stack
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, &w)| w > v)
            .map(|(i, v)| (i + 1, *v))
            .next()
            .unwrap_or((0, -1));
        
        if let Some(&j) = v_map.get(&v) {
            result[j] = pre_v;
        }

        monotonic_stack.truncate(index);
        monotonic_stack.push(v);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums1 = vec![4,1,2];
        let nums2 = vec![1,3,4,2];
        let res = vec![-1,3,-1];
        assert_eq!(next_greater_element(nums1, nums2), res);
    }

    #[test]
    fn test2() {
        let nums1 = vec![2,4];
        let nums2 = vec![1,2,3,4];
        let res = vec![3,-1];
        assert_eq!(next_greater_element(nums1, nums2), res);
    }
}