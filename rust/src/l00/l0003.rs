use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max = 0;
    let mut pl = 0;
    let mut pr = 0;
    let mut find = HashMap::<char, usize>::new();
    let s = s.chars().collect::<Vec<char>>();

    while pr < s.len(){
        let count = find.entry(s[pr]).or_insert(0);
        pr+=1;
        *count += 1;
        if *count == 1 {
            max = max.max(pr - pl);
        }

        while find[&s[pr-1]] >1 {
            let count = find.entry(s[pl]).or_insert(0);
            *count -= 1;
            pl+=1;
        }
    }
    max as i32
}