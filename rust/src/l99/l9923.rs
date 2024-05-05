use std::collections::HashMap;

pub fn min_anagram_length(s: String) -> i32 {
    let chars = s.chars().collect::<Vec<char>>();
    let count_chars_map = chars
        .iter()
        .fold(HashMap::<char, usize>::new(), |mut acc, c| {
            *acc.entry(*c).or_insert(0) += 1;
            acc
        });
    let count_chars = count_chars_map.iter().map(|v| *v.1).collect::<Vec<_>>();

    let single = count_chars
        .iter()
        .fold(0, |acc, &v| if acc == 0 { v } else { gcd(acc, v) });
    let unit= count_chars.iter().map(|v| v / single).sum::<usize>();
    let len = chars.len();

    for i in 1..=(len / unit){
        let uc = i * unit;
        if len % uc != 0 {
            continue;
        }

        let target_map = chars[0..uc]
            .iter()
            .fold(HashMap::<char, usize>::new(), |mut acc, c| {
                *acc.entry(*c).or_insert(0) += 1;
                acc
            });

        let mut matched = true;
        
        for j in 1..(len / uc){
            let current_map = chars[j * uc..(j + 1) * uc]
                .iter()
                .fold(HashMap::<char, usize>::new(), |mut acc, c| {
                    *acc.entry(*c).or_insert(0) += 1;
                    acc
                });
            if target_map != current_map {
                matched = false;
                break;
            }
        }

        if matched {
            return uc as i32;
        }
    }
    unreachable!();
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

// fn lcm(a: usize, b: usize) -> usize {
//     let v = a * b / gcd(a, b);
//     dbg!((a, b, v));
//     v
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(min_anagram_length("abba".to_string()), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(min_anagram_length("cdef".to_string()), 4);
    }

    #[test]
    fn test3() { 
        assert_eq!(min_anagram_length("aab".to_string()), 3);
    }

    #[test]
    fn test4() {
        assert_eq!(min_anagram_length("bbbaaabbb".to_string()), 9);
    }
}
