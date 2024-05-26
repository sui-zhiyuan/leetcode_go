pub fn compressed_string(word: String) -> String {


    let mut curr = '_';
    let mut count= 0;
    let mut result = "".to_string();
    for c in word.chars() {
        if c != curr  || count == 9 {
            if count >0 {
                result.push_str(&format!("{}{}", count, curr));
            }
            curr = c;
            count = 1;
        } else {
            count += 1;
        }
    }
    result.push_str(&format!("{}{}", count, curr));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let word = "abcde".to_string();
        assert_eq!(compressed_string(word), "1a1b1c1d1e");
    }

    #[test]
    fn test1() {
        let word = "aaaaaaaaaaaaaabb".to_string();
        assert_eq!(compressed_string(word), "9a5a2b");
    }

    #[test]
    fn test2() {
        let word = "aaaaaaaaaaaaaabbbbbbbbb".to_string();
        assert_eq!(compressed_string(word), "9a5a9b");
    }
}