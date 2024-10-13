use std::collections::HashSet;

pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
    let mut find = HashSet::new();

    let mut result = 0;
    for num in nums {
        if find.contains(&num) {
            result ^= num;
        } else {
            find.insert(num);
        }
    }

    result
}
