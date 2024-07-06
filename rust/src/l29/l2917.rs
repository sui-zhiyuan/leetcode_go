pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
    let mut values = vec![0; 31];
    for v in nums {
        for i in 0..31{
            if v & (1 << i) != 0 {
                values[i] += 1;
            }
        }
    }
    let mut result =0;
    for v in values.into_iter().rev() {
        result <<= 1;
        if v >= k {
            result |= 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(find_k_or(vec![7,12,9,8,9,15], 4), 9);
    }
}