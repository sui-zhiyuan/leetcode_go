pub fn count_odds(low: i32, high: i32) -> i32 {
    let low = low + if low % 2 == 0 { 1 } else { 0 };
    let high = high + if high % 2 == 0 { 1 } else { 2 };
    (high - low) / 2
}
