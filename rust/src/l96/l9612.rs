pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
    let positive = nums.len() / 2 + nums.len() % 2;
    let negative = nums.len() / 2;

    let mut squres = nums
        .into_iter()
        .map(|v| (v as i64) * (v as i64))
        .collect::<Vec<i64>>();

    squres.sort_unstable();

    squres[negative..].iter().sum::<i64>() - squres[..negative].iter().sum::<i64>()
}
