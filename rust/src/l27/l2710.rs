pub fn remove_trailing_zeros(num: String) -> String {
    let chars = num.chars().collect::<Vec<_>>();

    let last = chars
        .iter()
        .enumerate()
        .rev()
        .find(|v| *v.1 != '0')
        .map(|v| v.0)
        .unwrap_or(chars.len()-1);

    chars[..=last].iter().collect()
}
