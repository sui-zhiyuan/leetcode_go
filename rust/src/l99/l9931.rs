use std::collections::HashMap;

pub fn find_permutation_difference(s: String, t: String) -> i32 {
    let pos = s.chars().enumerate().fold(HashMap::new(), |mut acc , v| {
        acc.insert(v.1,v.0);
        acc
    });

    t.chars().enumerate().fold(0, |mut acc, v| {
        let &p = pos.get(&v.1).unwrap();
        acc += p.max(v.0) - p.min(v.0);
        acc
    }) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "abc".to_string();
        let t = "bac".to_string();
        assert_eq!(find_permutation_difference(s, t), 2);
    }

    #[test]
    fn test_2() {
        let s = "abcde".to_string();
        let t = "edbac".to_string();
        assert_eq!(find_permutation_difference(s, t), 12);
    }
}