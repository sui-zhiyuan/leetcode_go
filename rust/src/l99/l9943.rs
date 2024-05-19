pub fn sum_digit_differences(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let digits = nums[0].to_string().len();
    let mut count = vec![vec![0;10];digits];
    
    for mut n in nums{
        for i in 0..digits{
            count[i][(n%10) as usize] += 1;
            n /= 10;
        }
    }

    dbg!(&count);

    let mut result = 0;
    for i in 0..digits{
        result += combination(n as i32, 2);
        for j in 0..10{
            if count[i][j] > 1{
                result -= count[i][j] * (count[i][j] - 1) / 2;
            }
        }
    }

    result
}

fn combination(n: i32, k: i32) -> i64 {
    let mut res = 1i64;
    for i in 0..k {
        res *= (n - i) as i64;
        res /= (i + 1) as i64;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![13,23,12];
        let res = 4;
        assert_eq!(sum_digit_differences(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![10,10,10,10];
        let res = 0;
        assert_eq!(sum_digit_differences(nums), res);
    }

    #[test]
    fn test3() {
        let nums = vec![55, 44, 33, 22, 11];
        let res = 20;
        assert_eq!(sum_digit_differences(nums), res);
    }

    #[test]
    fn test4() {
        let nums = vec![51, 71, 17, 42];
        let res = 11;
        assert_eq!(sum_digit_differences(nums), res);
    }
}
