pub fn count_coprime(mat: Vec<Vec<i32>>) -> i32 {
    let mut primes = Vec::new();
    for i in 2usize..=160usize {
        let mut is_prime = true;
        for j in 2usize..i {
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(i);
        }
    }

    let mut primes_count = vec![0; 160];
    for &p in primes.iter() {
        for i in (0..160usize).rev() {
            if primes_count[i] != 0 && i * p < 160usize {
                primes_count[i * p] = primes_count[i] + 1;
            }
        }
        primes_count[p] = 1;
    }

    let mut result = 1i64;
    const MOD: i64 = 1_000_000_007;

    for v in mat.iter() {
        result = result * (v.len() as i64) % MOD;
    }

    let mut not_allow = 0i64;
    for p in 0..160usize {
        let ap_flag = primes_count[p];
        if ap_flag == 0 {
            continue;
        }

        let mut current = 1i64;

        for row in mat.iter() {
            let count = row.iter().filter(|&&x| x % (p as i32) == 0).count() as i64;
            current = current * count % MOD;
        }

        // if current != 0 {
        //     println!("{} {}" , p , current)
        // }

        if ap_flag % 2 == 1 {
            not_allow = (not_allow + current) % MOD;
        } else {
            not_allow = (not_allow + MOD - current) % MOD;
        }
    }

    ((result + MOD - not_allow) % MOD) as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::parse_grid;

    #[test]
    fn test_max_sum() {
        let input = parse_grid::<i32>("[[1,2],[3,4]]").unwrap();
        assert_eq!(3, count_coprime(input));
    }

    #[test]
    fn test_max_sum2() {
        let input = parse_grid::<i32>("[[1,1,1,4]]").unwrap();
        assert_eq!(3, count_coprime(input));
    }

    #[test]
    fn test_max_sum3() {
        let input = parse_grid::<i32>("[[150]]").unwrap();
        assert_eq!(0, count_coprime(input));
    }
}
