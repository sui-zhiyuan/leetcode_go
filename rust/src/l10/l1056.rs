use std::collections::HashMap;

pub fn confusing_number(n: i32) -> bool {
    let digit_map = [[0, 0], [1, 1], [6, 9], [8, 8], [9, 6]];
    let digit_map = digit_map
        .into_iter()
        .map(|v| (v[0], v[1]))
        .collect::<HashMap<_, _>>();

    let digits = n
        .to_string()
        .chars()
        .map(|v| v.to_digit(10).unwrap() as i32)
        .collect::<Vec<_>>();

    if digits.iter().any(|&v| !digit_map.contains_key(&v)) {
        return false;
    }

    let rev = digits.iter().rev().map(|&v| *digit_map.get(&v).unwrap()).collect::<Vec<_>>();

    if digits == rev {
        return false;
    }

    true
}
