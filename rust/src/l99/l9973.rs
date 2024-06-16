use std::collections::BTreeMap;

pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
    let powers = power.iter().fold(BTreeMap::<i32 , i32>::new(), |mut acc, &v| {
        *acc.entry(v).or_insert(0) += 1;
        acc
    }).into_iter().collect::<Vec<_>>();

    let mut dp = vec![0i64; powers.len()];

    for (i , &v) in powers.iter().enumerate() {
        let mut max = 0;
        let mut max_p = 0;
        for j in (0..i).rev(){
            if powers[j].0 + 2 >= v.0 {
                continue;
            }

            if max < dp[j] {
                max = dp[j];
                max_p = j;
            }

            if max_p > j+2 {
                break;
            }
        }

        dp[i] = max + powers[i].0 as i64 * powers[i].1 as i64;
    }

    *dp.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let power = vec![1,1,3,4];
        let result = 6;
        assert_eq!(maximum_total_damage(power), result);
    }

    #[test]
    fn test_2() {
        let power = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let result = 8;
        assert_eq!(maximum_total_damage(power), result);
    }

    #[test]
    fn test_3() {
        let power = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let result = 15;
        assert_eq!(maximum_total_damage(power), result);
    }

    #[test]
    fn test_4() {
        let power = vec![7,1,6,6];
        let result = 13;
        assert_eq!(maximum_total_damage(power), result);
    }
}