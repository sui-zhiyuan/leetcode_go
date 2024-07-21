pub fn minimum_levels(possible: Vec<i32>) -> i32 {
    let score = possible.into_iter().map(|v| if v == 0 { -1 } else { 1 }).collect::<Vec<i32>>();
    let n = score.len();
    let sum = score.iter().copied().sum::<i32>();

    let mut sum_l = 0;
    for (i , v) in score.into_iter().enumerate().take(n-1) {
        sum_l += v;
        let sum_r = sum - sum_l;
        if sum_l > sum_r{
            return i as i32 + 1;
        }
    }

    -1
}