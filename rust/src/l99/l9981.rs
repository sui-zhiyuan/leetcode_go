pub fn minimum_average(mut nums: Vec<i32>) -> f64 {
    nums.sort();
    assert!(nums.len() % 2 ==0);
    let mid = nums.len() / 2;

    let mut result = Vec::new();
    for i in 0..mid{
        result.push(nums[i] + nums[nums.len() - i - 1]);
    }

    result.into_iter().min().unwrap() as f64 / 2.0
}