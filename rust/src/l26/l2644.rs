pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
    let mut max_score = 0;
    let mut divider = divisors[0];

    for d in divisors {
        let score = nums.iter().filter(|&&v| v%d == 0).count();
        if score > max_score || (score == max_score && d < divider) {
            max_score = score;
            divider = d;
        }
    }

    divider
}