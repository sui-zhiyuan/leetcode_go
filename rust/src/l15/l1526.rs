pub fn min_number_operations(target: Vec<i32>) -> i32 {
    let mut prev = 0;
    let mut result = 0;

    for v in target {
        if prev < v {
            result += v - prev;
        }
        prev = v;
    }

    result
}
