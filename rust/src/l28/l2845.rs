pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
    let modulo_u = modulo as usize;
    let k_u = k as usize;
    let mut funny_count = vec![1];
    let mut result = 0;
    let mut count = 0usize;
    for &v in nums.iter() {
        if v % modulo == k {
            count += 1
        }

        if count >= modulo_u {
            count %= modulo_u;
        }

        let pre_count = (count + modulo_u - k_u) % modulo_u;
        if pre_count < funny_count.len() {
            result += funny_count[pre_count];
        }
        if count >= funny_count.len() {
            funny_count.resize(count + 1, 0);
        }
        funny_count[count] += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![3, 2, 4];
        assert_eq!(3, count_interesting_subarrays(nums, 2, 1));
    }

    #[test]
    fn test2() {
        let nums = vec![2, 2, 5];
        assert_eq!(2, count_interesting_subarrays(nums, 3, 2));
    }
}
