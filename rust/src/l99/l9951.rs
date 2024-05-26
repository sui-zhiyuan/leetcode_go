pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
    let mut result = 0;

    for v1 in nums1{
        for v2 in nums2.iter().copied() {
            if v1 % (v2 * k) == 0 {
                result += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums1 = vec![1,3,4];
        let nums2 = vec![1,3,4];
        let k = 1;
        assert_eq!(number_of_pairs(nums1, nums2, k), 5);
    }

    #[test]
    fn test1() {
        let nums1 = vec![1,2,4,12];
        let nums2 = vec![2,4];
        let k = 3;
        assert_eq!(number_of_pairs(nums1, nums2, k), 2);
    }
}