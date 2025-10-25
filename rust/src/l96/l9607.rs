pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
    let &last = nums2.last().unwrap();

    let mut result = 1i64; // append nearest to end
    let mut min_distance = i64::MAX;

    for i in 0..nums1.len() {
        result += (nums1[i] - nums2[i]).abs() as i64;

        if nums1[i] <= last && last <= nums2[i]  || nums1[i] >= last && last >= nums2[i] {
            min_distance = 0;
        }

        min_distance = min_distance.min((nums1[i]- last).abs() as i64);
        min_distance = min_distance.min((nums2[i]- last).abs() as i64);

    }

    result + min_distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_operations() {
        assert_eq!(min_operations(vec![2,8], vec![1,7,3]), 4);
    }
}