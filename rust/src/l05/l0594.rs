use std::collections::HashMap;

pub fn find_lhs(nums: Vec<i32>) -> i32 {
    let values = nums
        .into_iter()
        .fold(HashMap::<i32, i32>::new(), |mut map, v| {
            *map.entry(v).or_default() += 1;
            map
        });

    let mut result = 0;
    for (k, v) in values.iter() {
        let Some(next) = values.get(&(k + 1)) else {
            continue;
        };
        result = i32::max(result, v + next)
    }

    result
}
