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
            (State::None, true) => State::Mixed,
            (State::Lower, false) => State::Lower,
            (State::Lower, true) => State::Matched,
            (State::Matched, false) => State::Mixed,
            (State::Matched, true) => State::Matched,
            (State::Mixed, false) => State::Mixed,
            (State::Mixed, true) => State::Mixed,
        };
    }

    states.into_iter().filter(|&(_ , v)| v == State::Matched).count() as i32
}

#[derive(Debug, Clone, Copy, PartialEq , Eq)]
enum State{
    None,
    Lower,
    Matched,
    Mixed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_special_chars() {
        assert_eq!(number_of_special_chars("aaAbcBC".to_string()), 3);
        assert_eq!(number_of_special_chars("abc".to_string()), 0);
        assert_eq!(number_of_special_chars("AbBCab".to_string()), 0);
        assert_eq!(number_of_special_chars("cCceDC".to_string()), 0);
    }
}
