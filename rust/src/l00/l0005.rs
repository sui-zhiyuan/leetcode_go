pub fn longest_palindrome(s: String) -> String {
    let s = s.chars().collect::<Vec<char>>();

    let mut curr = &mut vec![0, 1];
    let mut next = &mut vec![];
    let mut result = &s[0..1];
    for (i, &v) in s.iter().enumerate().skip(1) {
        next.clear();
        next.push(0);
        next.push(1);
        for prev_len in curr.iter() {
            if i - prev_len > 0 && s[i - prev_len - 1] == v {
                next.push(prev_len + 2);
            }
        }

        let &max = next.last().unwrap();
        (curr,next) = (next,curr);

        if max > result.len() {
            result = &s[i + 1 - max..=i];
        }
    }

    result.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s: String = "aaa".to_string();
        assert_eq!(longest_palindrome(s), "aaa".to_string());
    }

    #[test]
    fn test2() {
        let s = "aaaaaa".to_string();
        assert_eq!(longest_palindrome(s), "aaaaaa".to_string());
    }
}
