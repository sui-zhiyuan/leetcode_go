pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
    let mut min = usize::MAX;
    let mut max = usize::MIN;
    for (i, v) in nums.iter().enumerate() {
        if is_prime(*v) {
            if i < min {
                min = i;
            }
            if i > max {
                max = i;
            }
        }
    }
    (max - min) as i32
}

fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }

    for i in 2..=(n as f64).sqrt() as i32 {
        if n % i == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(maximum_prime_difference(vec![4,2,9,5,3]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(maximum_prime_difference(vec![4,8,2,8]), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(maximum_prime_difference(vec![100, 10, 3, 7, 2]), 2);
    }
}