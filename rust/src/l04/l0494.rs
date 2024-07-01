use std::collections::HashMap;

pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let mut curr = &mut HashMap::new();
    let mut next = &mut HashMap::new();

    curr.insert(0, 1);
    for v in nums {
        for (k, c) in curr.iter() {
            *next.entry(k + v).or_insert(0) += c;
            *next.entry(k - v).or_insert(0) += c;
        }

        std::mem::swap(&mut curr, &mut next);
        next.clear();
    }

    curr.get(&target).copied().unwrap_or(0)
}
