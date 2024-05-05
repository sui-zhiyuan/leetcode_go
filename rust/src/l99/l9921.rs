pub fn is_valid(word: String) -> bool {
    let chars = word.chars().collect::<Vec<char>>();
    if chars.len() <=2 {
        return false;
    }

    if chars.iter().any(|c| !c.is_ascii_alphabetic() && !c.is_ascii_digit()) {
        return false;
    }

    let ua = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    if !chars.iter().any(|c| ua.contains(c)) {
        return false;
    }

    if !chars.iter().any(|c|  c.is_ascii_alphabetic() && !ua.contains(c)) {
        return false;
    }

    true
}

#[cfg(test)]
mod test{

    #[test]
    fn test_is_valid() {
        // assert_eq!(is_valid("234Adas".to_string()), true);
        // assert_eq!(is_valid("b3".to_string()), false);
        // assert_eq!(is_valid("a3$e".to_string()), false);
        // assert_eq!(is_valid("AOEIU".to_string()), false);
        // assert_eq!(is_valid("BCDF".to_string()), false);
    }

}