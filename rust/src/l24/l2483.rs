use std::iter::once;

pub fn best_closing_time(customers: String) -> i32 {
    let n = customers.chars().count();
    let sum_y = customers.chars().filter(|c| *c == 'Y').count();

    let mut pre_sum_y = 0;
    let mut pre_sum_n = 0;

    let mut min_cost = usize::MAX;
    let mut min_cost_time = 0;
    for (i, c) in customers.chars().chain(once(' ')).enumerate() {
        let cost = pre_sum_n + (sum_y - pre_sum_y);
        if cost < min_cost {
            min_cost = cost;
            min_cost_time = i;
        }
        if c == 'Y' {
            pre_sum_y += 1;
        } else {
            pre_sum_n += 1;
        }
    }
    min_cost_time as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameter_tests;
    struct Case {
        customers: &'static str,
        result: i32,
    }

    fn test(c: Case) {
        assert_eq!(c.result, best_closing_time(c.customers.to_owned()))
    }

    parameter_tests! {
        test,
        (case1: Case {
            customers: "YYNY",
            result: 2,
        }),
        (case2: Case {
            customers: "NNNNN",
            result: 0,
        }),
        (case3: Case {
            customers: "YYYY",
            result: 4,
        }),
    }
}
