use std::collections::HashMap;

pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
    let sig = numerator.signum() * denominator.signum();
    let sig = match sig {
        -1 => "-",
        0 | 1 => "",
        _ => unreachable!("invalid value"),
    };

    let mut numerator = (numerator as i64).abs();
    let denominator = (denominator as i64).abs();

    let mut digits = Vec::new();
    let mut repeat_checker = HashMap::new();

    let integer_part = numerator / denominator;
    numerator %= denominator;
    let mut repeat = None;

    for i in 0..10020 {
        if numerator == 0 {
            break;
        }
        if let Some(prev) = repeat_checker.insert(numerator, i) {
            repeat = Some(prev);
            break;
        }
        numerator *= 10;
        digits.push(numerator / denominator);
        numerator %= denominator;
    }

    if digits.is_empty() {
        format!("{sig}{integer_part}")
    } else if let Some(start) = repeat {
        format!(
            "{sig}{integer_part}.{}({})",
            sub::combine_digits(&digits[..start]),
            sub::combine_digits(&digits[start..])
        )
    } else {
        format!("{sig}{integer_part}.{}", sub::combine_digits(&digits[..]))
    }
}

mod sub{
    use std::fmt::Write;
    pub fn combine_digits(digits: &[i64]) -> String {
        let mut result = String::new();
        for d in digits {
            write!(result, "{}", d).unwrap();
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameter_tests;

    struct Case {
        numerator: i32,
        denominator: i32,
        result: String,
    }

    fn test(case: Case) {
        assert_eq!(
            case.result,
            fraction_to_decimal(case.numerator, case.denominator)
        )
    }

    parameter_tests! {
        test,
        (case1 : Case{numerator: 1, denominator: 2 , result: "0.5".to_string()}),
        (case2 : Case{numerator: 2, denominator: 1 , result: "2".to_string()}),
        (case3 : Case{numerator: 4, denominator: 333 , result: "0.(012)".to_string()}),
        (case4 : Case{numerator: -4, denominator: 333 , result: "-0.(012)".to_string()}),
    }
}
