use std::mem::swap;

pub fn has_same_digits(s: String) -> bool {
    let mut digits = s
        .chars()
        .map(|v| {
            assert!(v.is_ascii_digit());
            v as u32 - '0' as u32
        })
        .collect::<Vec<_>>();

    let mut next_digit = Vec::new();
    while digits.len() > 2 {
        for (i, v) in digits.iter().enumerate().take(digits.len() - 1) {
            next_digit.push((v + digits[i + 1]) % 10);
        }

        swap(&mut next_digit, &mut digits);
        next_digit.clear()
    }

    digits[0] == digits[1]
}
