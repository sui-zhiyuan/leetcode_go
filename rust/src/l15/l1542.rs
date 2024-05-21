use std::collections::HashMap;

pub fn longest_awesome(s: String) -> i32 {
    let s = s.chars().map(|v| v.to_digit(10).unwrap()).collect::<Vec<_>>();

    let mut min = HashMap::<u32 , usize>::new();

    let mut result = 0;
    let mut u = 0u32;
    for (i ,&v) in s.iter().enumerate() {
        u ^= 1 << v;
        min.entry(u).or_insert(i);
        if u.count_ones() <=1 {
            result = result.max(i + 1);
            continue;
        }
        for j in 0..10 {
            if let Some(&k) = min.get(&(u ^ (1 << j))) {
                result = result.max(i - k);
            }
        }
    }
    result as i32
}