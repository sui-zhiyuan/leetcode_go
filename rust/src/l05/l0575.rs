use std::collections::HashSet;

pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
    let kind = candy_type.iter().copied().collect::<HashSet<_>>().len();

    kind.min(candy_type.len() / 2) as i32
}
