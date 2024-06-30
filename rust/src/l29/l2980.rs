pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
    nums.iter().map(|v| if v %2 == 0 { 1 } else { 0 }).sum::<i32>() >= 2
}