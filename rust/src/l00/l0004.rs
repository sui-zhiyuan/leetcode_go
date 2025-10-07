pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let n = nums1.len() + nums2.len();
    let half = n.div_ceil(2);
    let p1 = binary_search(nums1.len(), |v| {
        if half < v {
            return true;
        }
        let u = half - v;
        u <= nums2.len() && (u == 0 || nums2[u - 1] <= nums1[v])
    });

    let p2 = half - p1;

    if n % 2 == 1 {
        let mut v1 = i32::MIN;
        if p1 > 0 {
            v1 = v1.max(nums1[p1 - 1]);
        }
        if p2 > 0 {
            v1 = v1.max(nums2[p2 - 1]);
        }
        v1 as f64
    } else {
        let mut v1 = i32::MIN;
        let mut v2 = i32::MAX;

        if p1 > 0 {
            v1 = v1.max(nums1[p1 - 1]);
        }
        if p2 > 0 {
            v1 = v1.max(nums2[p2 - 1]);
        }
        if p1 < nums1.len() {
            v2 = v2.min(nums1[p1]);
        }
        if p2 < nums2.len() {
            v2 = v2.min(nums2[p2]);
        }
        (v1 + v2) as f64 / 2.0
    }
}

fn binary_search(count: usize, mut check: impl FnMut(usize) -> bool) -> usize {
    if count == 0 {
        return 0;
    }
    let mut left = 0;
    let mut right = count;
    while left + 1 < right {
        let mid = left + (right - left) / 2;
        if check(mid) {
            right = mid;
        } else {
            left = mid;
        }
    }
    if check(left) {
        left
    } else {
        right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.0);
    }

    #[test]
    fn test2() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.5);
    }

    #[test]
    fn test3() {
        let nums1 = vec![0, 0];
        let nums2 = vec![0, 0];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 0.0);
    }

    #[test]
    fn test4() {
        let nums1 = vec![];
        let nums2 = vec![1];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 1.0);
    }

    #[test]
    fn test5() {
        let nums1 = vec![2];
        let nums2 = vec![];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.0);
    }

    #[test]
    fn test6() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2, 4];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.5);
    }

    #[test]
    fn test7() {
        let nums1 = vec![0, 0, 0, 0, 0];
        let nums2 = vec![-1, 0, 0, 0, 0, 0, 1];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 0.0);
    }

    #[test]
    fn test8() {
        let nums1 = vec![1, 2, 3, 4, 6, 7, 8];
        let nums2 = vec![5];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 4.5);
    }
}
