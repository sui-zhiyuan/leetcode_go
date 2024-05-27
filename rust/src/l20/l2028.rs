pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
    let sum = rolls.iter().sum::<i32>();
    let sum_a = mean * (rolls.len() as i32 + n) - sum;

    let sum_a = sum_a - n;
    if sum_a < 0 || sum_a > 5 * n {
        return vec![];
    }

    let count6 = sum_a / 5;
    let mid = sum_a % 5;
    let count1 = n - count6 - (if mid == 0 { 0 } else { 1 });
    let mut result = vec![1; count1 as usize];
    if mid > 0 {
        result.push(mid + 1);
    }
    result.append(&mut vec![6; count6 as usize]);
    result
}
