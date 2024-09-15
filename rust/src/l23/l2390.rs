pub fn remove_stars(s: String) -> String {
    let s = s.chars().collect::<Vec<_>>();

    let mut result = Vec::new();
    let mut end: usize = 0;
    for c in s {
        if c == '*' {
            end = end.saturating_sub(1);
            continue;
        }

        if result.len() == end {
            result.push(c);
            end +=1;
            continue;
        }

        result[end] = c;
        end += 1;
    }

    result[..end].iter().collect()
}