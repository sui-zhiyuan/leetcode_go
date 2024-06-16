pub fn count_complete_day_pairs(hours: Vec<i32>) -> i64 {
    let mut result = 0i64;
    let mut count = [0; 24];

    for &v in hours.iter() {
        count[(v % 24) as usize] += 1;
    }

    for i in 1..12 {
        result += count[i] * count[24 - i];
    }
    result += count[0] * (count[0] - 1) / 2;
    result += count[12] * (count[12] - 1) / 2;

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let hours = vec![12,12,30,24,24];
        let result = 2;
        assert_eq!(count_complete_day_pairs(hours), result);
    }

    #[test]
    fn test_2() {
        let hours = vec![72, 48, 24, 3];
        let result = 3;
        assert_eq!(count_complete_day_pairs(hours), result);
    }

    #[test]
    fn test_3() {
        let hours = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24,
        ];
        let result = 12;
        assert_eq!(count_complete_day_pairs(hours), result);
    }

    #[test]
    fn test_4() {
        let hours = vec![0, 0, 24];
        let result = 2;
        assert_eq!(count_complete_day_pairs(hours), result);
    }
}
