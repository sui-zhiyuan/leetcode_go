pub fn flower_game(n: i32, m: i32) -> i64 {
    let (even_n, odd_n) = number_count(n);
    let (even_m, odd_m) = number_count(m);

    even_n * odd_m + odd_n * even_m
}

// return even number count and odd number count in 1..=n
fn number_count(v: i32) -> (i64, i64) {
    let even_count = v / 2;

    let odd_count = v - even_count;
    (even_count as i64, odd_count as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(3, flower_game(3, 2));
    }

    #[test]
    fn test2() {
        assert_eq!(0, flower_game(1, 1));
    }
}
