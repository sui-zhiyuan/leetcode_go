pub fn smallest_number(n: i32) -> i32 {
    let first_one = size_of::<i32>() * 8 - n.leading_zeros() as usize;
    (1 << first_one) - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameter_tests;

    struct Case {
        n: i32,
        expect: i32,
    }

    fn test(c: Case) {
        assert_eq!(c.expect, smallest_number(c.n))
    }

    parameter_tests! {
        test,
        (case1 : Case { n: 5, expect: 7 }),
        (case2 : Case { n: 10, expect: 15 }),
        (case3 : Case { n: 3, expect: 3 }),
        (case4 : Case { n: 0, expect: 0 }),
    }
}
