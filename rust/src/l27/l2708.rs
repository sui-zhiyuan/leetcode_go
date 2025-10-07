pub fn max_strength(nums: Vec<i32>) -> i64 {
    let max_negative = nums.iter().filter(|&&x| x < 0).max().copied();
    let has_zero = nums.contains(&0);

    let mut results = Vec::new();

    let all = nums
        .iter()
        .filter(|&&x| x != 0)
        .fold(1i64, |acc, &x| acc * x as i64);
    let total_count = nums.iter().filter(|&&x| x != 0).count();
    if total_count >= 1 {
        results.push(all);
    }

    if has_zero {
        results.push(0);
    }

    if let Some(max_negative) = max_negative {
        let non_zero_count = nums.iter().filter(|&&x| x != 0).count();
        if non_zero_count > 1 {
            results.push(all / max_negative as i64);
        }
    }

    results.into_iter().max().unwrap()
}
