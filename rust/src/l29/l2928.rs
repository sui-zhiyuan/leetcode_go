pub fn distribute_candies(n: i32, limit: i32) -> i32 {
    let mut result = 0;
    let start = 0.max(n - 2 * limit);
    let end = limit.min(n);
    for i in start..=end {
        let rest = n - i;
        if limit.min(rest) >= 0.max(rest - limit) {
            result += limit.min(rest) - 0.max(rest - limit) + 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(distribute_candies(5, 2), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(distribute_candies(3, 3), 10);
    }
}
