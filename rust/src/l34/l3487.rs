use std::collections::HashSet;

pub fn max_sum(nums: Vec<i32>) -> i32 {
    if nums.iter().copied().all(|v| v < 0) {
        return nums.iter().copied().max().unwrap();
    }

    let dedup = nums.into_iter().filter(|&v| v > 0).collect::<HashSet<_>>();

    dedup.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(15, max_sum(vec![1, 2, 3, 4, 5]))
    }

    #[test]
    fn test2() {
        assert_eq!(1, max_sum(vec![1, 1, 0, 1, 1]))
    }

    #[test]
    fn test3() {
        assert_eq!(-100, max_sum(vec![-100]))
    }
}
