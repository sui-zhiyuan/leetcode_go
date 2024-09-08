pub fn clear_digits(s: String) -> String {
    let mut result = Vec::new();
    let mut top: usize = 0;
    for c in s.chars() {
        if c.is_ascii_digit() {
            top = top.saturating_sub(1);
            continue;
        }

        if top == result.len() {
            result.push(c);
            top += 1;
        } else {
            result[top] = c;
            top += 1;
        }
    }

    result[..top].iter().collect()
}
