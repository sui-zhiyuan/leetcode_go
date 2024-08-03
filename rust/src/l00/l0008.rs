// cspell:ignore atoi
pub fn my_atoi(s: String) -> i32 {
    let mut has_number = false;
    let mut sign = None;
    let mut value:i32 = 0;

    'outer: for c in s.chars() {
        match c {
            ' ' => {
                if has_number || sign.is_some() {
                    break 'outer;
                }
            }
            '+' => {
                if has_number {
                    break 'outer;
                }
                if sign.is_some() {
                    break 'outer;
                }
                sign = Some(1);
            }
            '-' => {
                if has_number {
                    break 'outer;
                }
                if sign.is_some() {
                    break 'outer;
                }
                sign = Some(-1);
            }
            '0'..='9' => {
                has_number = true;
                if sign.is_none() {
                    sign = Some(1);
                }

                let d = c.to_digit(10).unwrap() as i32;
                let d = sign.unwrap() * d;
                value = value.saturating_mul(10);
                value = value.saturating_add(d);
            }
            _ => {
                break 'outer;
            }
        }
    }
    value
}