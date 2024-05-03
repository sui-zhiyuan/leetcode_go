pub fn average(salary: Vec<i32>) -> f64 {
    let (max, min, sum, count) = salary
        .into_iter()
        .fold((i32::MIN, i32::MAX, 0, 0), |(max, min, sum, count), s| {
            (max.max(s), min.min(s), sum + s, count + 1)
        });
    ((sum - max - min) as f64) / ((count - 2) as f64)
}
