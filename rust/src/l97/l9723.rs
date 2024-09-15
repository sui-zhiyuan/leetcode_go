use std::{collections::HashMap, vec};

pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
    let target = target.chars().collect::<Vec<_>>();

    let mut matches = HashMap::new();
    for w in words {
        let w = w.chars().collect::<Vec<_>>();
        let pi = prefix_function(&w);
        
        let mut pic = vec![0 ; target.len()];
        for i in 0..target.len() {
            let mut j = if i == 0 {0} else {pic[i - 1]};
            while j > 0 && ( j >= w.len() || target[i] != w[j]) {
                j = if j > pi.len() {0} else { pi[j - 1]};
            }
            if target[i] == w[j] {
                j += 1;
            }
            pic[i] = j;
        }

        for (i , &v) in pic.iter().enumerate() {
            if v == 0 {
                continue;
            }
            let start  = i +1 -v;
            let end = i +1;
            let m = matches.entry(end).or_insert(start);
            *m = (*m).min(start);
        }
    }

    let mut reach = vec![i32::MAX; target.len()+1];
    reach[target.len()] = 0;
    for i in (1..=target.len()).rev() {
        if reach[i] == i32::MAX {
            continue;
        }
        let Some(&start) = matches.get(&i) else {
            continue;
        };
        for j in start..i {
            reach[j] = reach[j].min(reach[i] + 1);
        }
    }

    if reach[0] == i32::MAX {
        -1
    } else {
        reach[0]
    }
}


fn prefix_function(s: &[char]) -> Vec<usize> {
    let n = s.len();
    let mut pi = vec![0; n];
    for i in 1..n {
        let mut j = pi[i - 1];
        while j > 0 && s[i] != s[j] {
            j = pi[j - 1];
        }
        if s[i] == s[j] {
            j += 1;
        }
        pi[i] = j;
    }
    pi
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let words = ["abc","aaaaa","bcdef"];
        let words = words.iter().map(|&s| s.to_string()).collect();
        let target = "aabcdabc".to_string();
        let res = min_valid_strings(words, target);
        assert_eq!(res, 3);
    }

    #[test]
    fn test2() {
        let words = ["b","ccacc","a"];
        let words = words.iter().map(|&s| s.to_string()).collect();
        let target = "cccaaaacba".to_string();
        let res = min_valid_strings(words, target);
        assert_eq!(res, 8);
    }
}