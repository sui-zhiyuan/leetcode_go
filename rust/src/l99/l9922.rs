use std::{cmp, collections::HashMap};

pub fn minimum_operations_to_make_k_periodic(word: String, k: i32) -> i32 {
    let word = word.chars().collect::<Vec<char>>();
    let mut matched = HashMap::<String, i32>::new();
    for i in 0..(word.len() / k as usize) {
        let sub_str = word[i * k as usize..(i + 1) * k as usize]
            .iter()
            .collect::<String>();
        *matched.entry(sub_str).or_insert(0) += 1;
    }
    let mut matched = matched.into_iter().collect::<Vec<_>>();
    matched.sort_by_key(|v| cmp::Reverse(v.1));

    matched.iter().skip(1).map(|v| v.1).sum::<i32>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            minimum_operations_to_make_k_periodic("leetcodeleet".to_string(), 4),
            1
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            minimum_operations_to_make_k_periodic("leetcoleet".to_string(), 2),
            3
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            minimum_operations_to_make_k_periodic("aa".to_string(), 1),
            0
        );
    }
}
