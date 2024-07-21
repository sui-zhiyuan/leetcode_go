use std::collections::HashSet;

pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let s_nums1 = nums1.iter().copied().collect::<HashSet<_>>();
    let s_nums2 = nums2.iter().copied().collect::<HashSet<_>>();

    let mut result1 = 0;
    let mut result2 = 0;

    for v in nums1 {
        if s_nums2.contains(&v) {
            result1 +=1;
        }
    }

    for v in nums2 {
        if s_nums1.contains(&v) {
            result2 +=1;
        }
    }

    vec![result1, result2]
}