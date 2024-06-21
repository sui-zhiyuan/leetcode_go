// cspell:ignore logf
pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
    let mut count = 0;

    for (i, &v1) in nums.iter().enumerate() {
        let d = 10f64.powf((v1 as f64).log10().floor()) as i32;
        let v1 = v1 / d;
        for &v2 in nums.iter().skip(i + 1) {
            let v2 = v2 % 10;
            if coprime(v1, v2) {
                count += 1;
            }
        }
    }

    count
}

fn coprime(a: i32, b: i32) -> bool {
    for i in 2..=a.min(b) {
        if a % i == 0 && b % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![2, 5, 1, 4];
        assert_eq!(count_beautiful_pairs(nums), 5);
    }

    #[test]
    fn test2() {
        let nums = vec![84, 91, 18, 59, 27, 9, 81, 33, 17, 58];
        assert_eq!(count_beautiful_pairs(nums), 37);
    }
}
