use std::collections::HashMap;

pub fn sum_divisible_by_k(nums: Vec<i32>, k: i32) -> i32 {
    let count = nums
        .into_iter()
        .fold(HashMap::<i32, i32>::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });

    let mut sum = 0;
    for (v, count) in count {
        if count % k == 0 {
            sum += v * count;
        }
    }
    
    sum
}
