use std::collections::HashMap;

pub fn maximum_length(s: String) -> i32 {
    let mut total_count = HashMap::<(char, usize), usize>::new();
    let mut last_char = '_';
    let mut char_count = 0usize;
    for c in s.chars() {
        if c == last_char {
            char_count += 1;
        } else {
            last_char = c;
            char_count = 1;
        }

        for i in 1..=char_count {
            *total_count.entry((c, i)).or_insert(0) += 1;
        }
    }

    total_count
        .iter()
        .filter(|&(_, &c)| c >= 3)
        .map(|(&(_, c), _)| c as i32)
        .max()
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "aaaa".to_string();
        assert_eq!(maximum_length(s), 2);
    }

    #[test]
    fn test1() {
        let s = "abcdef".to_string();
        assert_eq!(maximum_length(s), -1);
    }

    #[test]
    fn test2() {
        // cspell:ignore abcaba
        let s = "abcaba".to_string();
        assert_eq!(maximum_length(s), 1);
    }
}
