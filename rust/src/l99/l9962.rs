use std::mem;

pub fn value_after_k_seconds(n: i32, k: i32) -> i32 {
    let n = n as usize;
    let curr = &mut vec![1u64 ; n];
    let next = &mut vec![0u64 ; n];
    const MOD: u64 = 1_000_000_007;
    for i in 0..k {
        let mut v = 0;
        for j in 0..n {
            v = (v + curr[j]) % MOD;
            next[j] = v;
        }
        mem::swap(curr, next);
    }
    curr[n-1] as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(value_after_k_seconds(4, 5), 56);
    }

    #[test]
    fn test2() {
        assert_eq!(value_after_k_seconds(5, 3), 35);
    }
}