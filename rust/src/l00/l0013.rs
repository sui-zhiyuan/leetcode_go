pub fn roman_to_int(s: String) -> i32 {
    let chars = [('I', 'V'), ('X', 'L'), ('C', 'D'), ('M', '_'), ('_', '_')];
    let map = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];

    let mut result = 0;
    let s = s.chars().collect::<Vec<_>>();
    let mut i = 0;
    let mut digit = 3;
    while i < s.len() {
        let valid_char = [chars[digit].0, chars[digit].1, chars[digit + 1].0];
        let mut next = i;
        while next < s.len() && valid_char.contains(&s[next]) {
            next += 1;
        }

        let curr_dig = s[i..next]
            .iter()
            .map(|&c| {
                if c == chars[digit].0 {
                    'I'
                } else if c == chars[digit].1 {
                    'V'
                } else if c == chars[digit + 1].0 {
                    'X'
                } else {
                    panic!("invalid char")
                }
            })
            .collect::<String>();

        let curr_dig = map.iter().position(|&v| v == curr_dig).unwrap() as i32;
        result += curr_dig * 10_i32.pow(digit as u32);
        
        i = next;
        digit = digit.saturating_sub(1);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(roman_to_int("III".to_owned()), 3)
    }

    #[test]
    fn test2() {
        assert_eq!(roman_to_int("LVIII".to_owned()), 58)
    }
}
