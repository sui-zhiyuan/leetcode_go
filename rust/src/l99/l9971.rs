pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
    let mut result = 0;

    for (i, &v1) in hours.iter().enumerate() {
        for (_j, &v2) in hours.iter().enumerate().skip(i + 1) {
            if (v1 + v2) % 24 == 0 {
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
    fn test_1() {
        let hours = vec![12, 12, 30, 24, 24];
        let result = 2;
        assert_eq!(count_complete_day_pairs(hours), result);
    }

    #[test]
    fn test_2() {
        let hours = vec![72, 48, 24, 3];
        let result = 3;
        assert_eq!(count_complete_day_pairs(hours), result);
    }
}
