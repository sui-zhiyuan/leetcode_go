use std::collections::HashMap;

pub fn relocate_marbles(nums: Vec<i32>, move_from: Vec<i32>, move_to: Vec<i32>) -> Vec<i32> {
    let mut state = HashMap::<usize, i32>::new();
    for v in nums {
        *state.entry(v as usize).or_insert(0) += 1;
    }

    for (s, t) in move_from.iter().zip(move_to.iter()) {
        let s = *s as usize;
        let t = *t as usize;
        let v = *state.entry(s).or_insert(0);
        *state.entry(s).or_insert(0) = 0;
        *state.entry(t).or_insert(0) += v;
    }

    let mut result = state
        .into_iter()
        .filter_map(|(k, v)| if v > 0 { Some(k as i32) } else { None })
        .collect::<Vec<_>>();
    result.sort();
    result
}
