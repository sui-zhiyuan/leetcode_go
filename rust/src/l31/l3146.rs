use std::collections::HashMap;

pub fn find_permutation_difference(s: String, t: String) -> i32 {
    let s = s
        .chars()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect::<HashMap<_, _>>();

    let mut result = 0;
    for (i, t) in t.chars().enumerate() {
        let ps = s.get(&t).unwrap();
        result += ((i as i32) - (*ps as i32)).abs();
    }

    result
}
