pub fn can_alice_win(nums: Vec<i32>) -> bool {
    let sum = nums.iter().sum::<i32>();

    let mut sum1 = 0;
    let mut sum2 = 0;
    for v in nums {
        if digts(v) == 1 {
            sum1 += v;
        }
        if digts(v) == 2 {
            sum2 += v;
        }
    }

    sum1 > (sum - sum1) || sum2 > (sum - sum2)
}

fn digts(n: i32) -> i32 {
    let mut n = n;
    let mut res = 0;
    while n > 0 {
        res += 1;
        n /= 10;
    }
    res
}
