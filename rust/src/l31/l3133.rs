pub fn min_end(n: i32, x: i32) -> i64 {
    let mut n = n as u64 - 1;
    let mut x = x as u64;

    let mut curr = 1u64;
    while n > 0 {
        while x & curr == curr {
            curr <<= 1;
        }

        if n & 1 == 1 {
            x |= curr;
        }
        n >>= 1;
        curr <<= 1;
    }

    x as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(min_end(2, 7), 15);
    }
}
