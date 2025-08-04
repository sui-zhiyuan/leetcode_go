pub fn longest_common_prefix(strs: Vec<String>) -> String {
    strs.into_iter()
        .reduce(common_prefix)
        .expect("at least 1 element")
}

fn common_prefix(left: String, right: String) -> String {
    let mut result = String::new();

    let mut left = left.chars();
    let mut right = right.chars();
    loop {
        match (left.next(), right.next()) {
            (None, _) => break,
            (_, None) => break,
            (Some(l), Some(r)) if l == r => {
                result.push(l);
            }
            _ => break,
        }
    }

    result
}
