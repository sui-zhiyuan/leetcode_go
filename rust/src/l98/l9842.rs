pub fn non_special_count(l: i32, r: i32) -> i32 {
    let max = (r as f64).sqrt() as usize;
    let primes = all_prims(max);

    let mut total = r - l + 1;
    let sqr_l = (l as f64).sqrt() as i32;
    let sqr_r = (r as f64).sqrt() as i32;
    for i in sqr_l..=sqr_r {
        if i * i >= l && i * i <= r && primes[i as usize] {
            total -= 1;
        }
    }
    total
}

fn all_prims(n: usize) -> Vec<bool> {
    let mut result = vec![true; n + 1];
    result[0] = false;
    result[1] = false;
    let mut primes = vec![];
    for i in 2..=n {
        if result[i] {
            primes.push(i);
        }

        for &p in primes.iter() {
            if i * p > n {
                break;
            }

            result[i * p] = false;
            if i % p == 0 {
                break;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(non_special_count(4, 16), 11);
    }
}
