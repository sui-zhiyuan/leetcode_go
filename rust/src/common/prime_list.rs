use std::mem::swap;

pub fn prime_list(max: i32) -> Vec<i32> {
    let mut curr = (2..=max).collect::<Vec<_>>();
    let mut next = Vec::<i32>::new();

    let mut i = 0;
    while i < curr.len() {
        let mut k = 0;
        for j in i..curr.len() {
            let next_filtered = curr[i].checked_mul(curr[j]).unwrap_or(i32::MAX);

            while k < curr.len() && curr[k] < next_filtered {
                next.push(curr[k]);
                k += 1;
            }
            if k == curr.len() {
                break;
            }

            assert_eq!(curr[k], next_filtered);
            k += 1;
        }

        swap(&mut curr, &mut next);
        next.clear();
        i += 1;
    }

    curr
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test() {
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
        assert_eq!(prime_list(50), primes);
    }
}
