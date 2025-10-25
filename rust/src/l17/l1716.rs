pub fn total_money(n: i32) -> i32 {
    let base_week_sum = [0, 1, 3, 6, 10, 15, 21, 28];
    let increase_sum = [0, 1, 2, 3, 4, 5, 6, 7];

    let complete_week = n / 7;
    let complete_week_amount = base_week_sum[7] * complete_week
        + increase_sum[7] * complete_week * (complete_week - 1) / 2;

    let last_week_day = n % 7;
    let last_week_amount = base_week_sum[last_week_day as usize]
        + increase_sum[last_week_day as usize] * complete_week;

    complete_week_amount + last_week_amount
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameter_tests;

    struct Case {
        input: i32,
        expect: i32,
    }

    fn test(c: Case) {
        assert_eq!(c.expect, total_money(c.input))
    }

    parameter_tests! {
        test,
        (case1: Case {
            input: 4,
            expect: 10,
        }),
        (case2: Case {
            input: 10,
            expect: 37,
        }),
        (case3: Case {
            input: 20,
            expect: 96,
        }),
    }
}
