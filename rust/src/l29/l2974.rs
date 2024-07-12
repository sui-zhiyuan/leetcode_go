pub fn number_game(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort();
    for i in 0..nums.len() / 2 {
        nums.swap(2 * i, 2 * i + 1)
    }
    nums
}
