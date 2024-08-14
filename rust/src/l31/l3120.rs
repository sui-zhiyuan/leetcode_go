use std::collections::HashMap;

pub fn number_of_special_chars(word: String) -> i32 {
    let mut states = HashMap::<char, State>::new();

    for c in word.chars() {
        if !c.is_alphabetic() {
            panic!("Invalid character")
        }

        let is_upper = c.is_ascii_uppercase();
        let lower_char = c.to_ascii_lowercase();
        let state = states.entry(lower_char).or_insert(State::None);
        *state = match (*state , is_upper) {
            (State::None, false) => State::Lower,
            (State::None, true) => State::Upper,
            (State::Lower, false) => State::Lower,
            (State::Lower, true) => State::Matched,
            (State::Upper, false) => State::Matched,
            (State::Upper, true) => State::Upper,
            (State::Matched, false) => State::Matched,
            (State::Matched, true) => State::Matched,
        };
    }

    states.into_iter().filter(|&(_ , v)| v == State::Matched).count() as i32
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    None,
    Lower,
    Upper,
    Matched,
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
