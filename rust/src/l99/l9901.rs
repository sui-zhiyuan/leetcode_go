use std::collections::HashMap;

pub fn number_of_special_chars(word: String) -> i32 {
    let mut states = HashMap::<char, State>::new();
    let mut result = 0;

    for c in word.chars() {
        if !c.is_alphabetic() {
            panic!("Invalid character")
        }
        
        if c.is_ascii_lowercase() {
            let state = states.entry(c).or_insert(State::None);
            *state = match *state {
                State::None => State::Lower,
                State::Upper => {
                    result += 1;
                    State::Added
                },
                other => other,
            };
            
        }
        if c.is_uppercase() {
            let state = states.entry(c.to_ascii_lowercase()).or_insert(State::None);
            *state = match *state {
                State::None => State::Upper,
                State::Lower => {
                    result += 1;
                    State::Added
                },
                other => other,
            };
        }
    }

    result
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    None,
    Lower,
    Upper,
    Added,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_special_chars() {
        assert_eq!(number_of_special_chars("aaAbcBC".to_string()), 3);
        assert_eq!(number_of_special_chars("abc".to_string()), 0);
        assert_eq!(number_of_special_chars("abBCab".to_string()), 1);
    }
}
