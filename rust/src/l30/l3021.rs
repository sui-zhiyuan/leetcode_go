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
    use crate::parameter_tests;

    struct TestCase {
        result: i64,
        n: i32,
        m: i32,
    }

    fn test(p: TestCase) {
        assert_eq!(p.result, flower_game(p.n, p.m))
    }

    parameter_tests! {
        test,
        (case_1: TestCase{result:3, n: 3 , m: 2}),
        (case_2: TestCase{result:0, n: 1 , m: 1}),
    }
}
