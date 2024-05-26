use std::collections::HashMap;

pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let mut count = HashMap::<i32, i64>::new();

    for v in nums1 {
        if v % k != 0 {
            continue;
        }
        let v = v / k;
        for j in 1..=v {
            // if exchange 13-15 and 16-17, j*j will overflow. 
            // test 2 will trigger overflow in debug mode.
            // test 2 will return 2 in release mode.
            if j * j > v {
                break;
            }
            if v % j != 0 {
                continue;
            }
            *count.entry(j).or_insert(0) += 1;

            let j1 = v / j;
            if j1 > j {
                *count.entry(j1).or_insert(0) += 1;
            }
        }
    }

    nums2
        .into_iter()
        .map(|v| *count.get(&v).unwrap_or(&0))
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums1 = vec![1, 3, 4];
        let nums2 = vec![1, 3, 4];
        let k = 1;
        assert_eq!(number_of_pairs(nums1, nums2, k), 5);
    }

    #[test]
    fn test1() {
        let nums1 = vec![1, 2, 4, 12];
        let nums2 = vec![2, 4];
        let k = 3;
        assert_eq!(number_of_pairs(nums1, nums2, k), 2);
    }

    #[test]
    fn test2() {
        let nums1 = vec![206917];
        let nums2 = vec![206917];
        let k = 1;
        assert_eq!(number_of_pairs(nums1, nums2, k), 1);
    }
}
