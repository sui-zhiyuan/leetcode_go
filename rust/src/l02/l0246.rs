use std::collections::HashMap;

// cspell:ignore strobogrammatic
pub fn is_strobogrammatic(num: String) -> bool {
    let num = num.chars().collect::<Vec<_>>();
    let n = num.len();

    let mut rev = HashMap::new();
    rev.insert('0', '0');
    rev.insert('1', '1');
    rev.insert('2', 'x');
    rev.insert('3', 'x');
    rev.insert('4', 'x');
    rev.insert('5', 'x');
    rev.insert('6', '9');
    rev.insert('7', 'x');
    rev.insert('8', '8');
    rev.insert('9', '6');

    for i in 0..(n / 2 + 1) {
        if num[i] != rev[&num[n - 1 - i]] {
            return false;
        }
    }

    true
}
