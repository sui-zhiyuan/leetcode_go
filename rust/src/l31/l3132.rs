pub fn minimum_added_integer(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
    nums1.sort();
    nums2.sort();

    let mut result = i32::MAX;

    let d1 = nums2[0] - nums1[0];
    if check(&nums1, &nums2, d1, 2) {
        result = result.min(d1);
    }

    let d2 = nums2[0] - nums1[1];
    if check(&nums1[1..], &nums2, d2, 1) {
        result = result.min(d2);
    }

    let d3 = nums2[0] - nums1[2];
    if check(&nums1[2..], &nums2, d3, 0) {
        result = result.min(d3);
    }

    result
}

fn check(nums1: &[i32], nums2: &[i32], diff: i32, max_miss: usize) -> bool {
    let mut miss = 0;
    let mut i = 0;
    let mut j = 0;

    while i < nums1.len() && j < nums2.len() {
        if nums1[i] + diff == nums2[j] {
            i += 1;
            j += 1;
            continue;
        }

        i += 1;
        miss += 1;
    }

    miss += nums1.len() - i;
    miss += nums2.len() - j;

    miss == max_miss
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums1 = vec![4, 20, 16, 12, 8];
        let nums2 = vec![14, 18, 10];
        assert_eq!(minimum_added_integer(nums1, nums2), -2);
    }

    #[test]
    fn test2() {
        let nums1 = vec![4,6,3,1,4,2,10,9,5];
        let nums2 = vec![5,10,3,2,6,1,9];
        assert_eq!(minimum_added_integer(nums1, nums2), 0);
    }
}
