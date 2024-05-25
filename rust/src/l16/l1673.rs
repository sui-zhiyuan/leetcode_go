pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;

    let mut result = Vec::new();
    for ( i , v) in nums.iter().enumerate() {
        while !result.is_empty() && *result.last().unwrap() > *v && result.len() -1 + nums.len() -i >=k {
            result.pop();
        }
        result.push(*v);
    }

    result[..k].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![3, 5, 2, 6];
        let k = 2;
        assert_eq!(most_competitive(nums, k), vec![2, 6]);
    }

    #[test]
    fn test1() {
        let nums = vec![2, 4, 3, 3, 5, 4, 9, 6];
        let k = 4;
        assert_eq!(most_competitive(nums, k), vec![2, 3, 3, 4]);
    }

    #[test]
    fn test2() {
        let nums = vec![3, 2, 1];
        let k = 2;
        assert_eq!(most_competitive(nums, k), vec![2,1]);
    }
}