pub fn maximum69_number(num: i32) -> i32 {
    let mut d_num = num;
    let mut digits = Vec::with_capacity(4);

    while d_num > 0 {
        digits.push(d_num % 10);
        d_num /= 10;
    }

    let Some(pos) = digits.iter().rposition(|v| *v == 6) else {
        return num;
    };

    digits[pos] = 9;

    let mut result = 0;

    for v in digits.into_iter().rev() {
        result *= 10;
        result += v
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(9969, maximum69_number(9669))
    }
}
