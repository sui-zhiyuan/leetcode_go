pub fn lex_smallest(s: String) -> String {
    let mut result = s.clone();
    let chars = s.chars().collect::<Vec<char>>();

    for k in 0..=chars.len() {
        let mut new = Vec::new();
        new.extend(chars[..k].iter().rev().copied());
        new.extend_from_slice(&chars[k..]);
        let new = new.iter().collect::<String>();
        result = result.min(new);


        let mut new = Vec::new();
        let k = chars.len() - k;
        new.extend_from_slice(&chars[..k]);
        new.extend(chars[k..].iter().rev().copied());
        let new = new.iter().collect::<String>();
        result = result.min(new);
    }
    result
}

#[test]
fn test_smallest() {
    assert_eq!(lex_smallest("dcab".to_string()), "acdb".to_string());
}


#[test]
fn test_smallest2() {
    assert_eq!(lex_smallest("abba".to_string()), "aabb".to_string());
}

#[test]
fn test_smallest3() {
    assert_eq!(lex_smallest("kmko".to_string()), "kmko".to_string());
}

#[test]
fn test_smallest4() {
    assert_eq!(lex_smallest("lk".to_string()), "kl".to_string());
}