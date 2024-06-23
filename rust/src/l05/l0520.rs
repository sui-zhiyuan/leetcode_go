pub fn detect_capital_use(word: String) -> bool {
    let chars = word.chars().collect::<Vec<char>>();

    chars.iter().all(|&c| c.is_uppercase()) 
    || chars.iter().all(|&c| c.is_lowercase()) 
    || (chars[0].is_uppercase() && chars[1..].iter().all(|&c| c.is_lowercase()))
}