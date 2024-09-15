use std::collections::HashSet;

pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut find = HashSet::<i32>::new();

    let mut result = Vec::new();
    for v in nums {
        if find.contains(&v) {
            result.push(v);
        } else {
            find.insert(v);
        }
    }

    result
}