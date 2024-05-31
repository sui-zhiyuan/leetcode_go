use std::{cmp::Reverse, collections::HashMap};

pub fn maximum_length(s: String) -> i32 {
    let mut total_count = HashMap::<char, Vec<usize>>::new();

    let mut curr = '\0';
    let mut count: usize = 0;
    for c in s.chars() {
        if c == curr {
            count += 1;
            continue;
        }

        if curr == '\0' {
            curr = c;
            count = 1;
            continue;
        }

        let t = total_count.entry(curr).or_default();
        t.push(count);
        t.sort_by_key(|v| Reverse(*v));
        if t.len() > 3 {
            t.pop();
        }

        curr = c;
        count = 1;
    }

    let t = total_count.entry(curr).or_default();
    t.push(count);
    t.sort_by_key(|v| Reverse(*v));
    if t.len() > 3 {
        t.pop();
    }


    let mut max = 0;
    for (_ , v) in total_count {
        if !v.is_empty() && v[0] > max +2{
            max = v[0] - 2;
        }

        if v.len() >=2 && v[0] == v[1] && v[0] > max +1 {
            max = v[0] - 1;
        }

        if v.len() >=2 && v[0] > v[1] && v[1] > max {
            max = v[1];
        }

        if v.len() >=3 && v[2] > max{
            max = v[2];
        }
    }
    if max == 0 {
        -1
    } else {
        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "aaaa".to_string();
        assert_eq!(maximum_length(s), 2);
    }

    #[test]
    fn test1() {
        let s = "abcdef".to_string();
        assert_eq!(maximum_length(s), -1);
    }

    #[test]
    fn test2() {
        // cspell:ignore abcaba
        let s = "abcaba".to_string();
        assert_eq!(maximum_length(s), 1);
    }

    #[test]
    fn test3() {
        // cspell:ignore abcabc
        let s = "abcabc".to_string();
        assert_eq!(maximum_length(s), -1);
    }
}
