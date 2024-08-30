pub fn sum_digit_differences(nums: Vec<i32>) -> i64 {
    let n = nums[0].to_string().len();

    let mut count = vec![[0; 10]; n];
    let mut result = 0;
    for (i, &num) in nums.iter().enumerate() {
        let mut num = num;
        for c in count.iter_mut() {
            let v = &mut c[(num % 10) as usize];
            result += (i - *v) as i64;
            *v += 1;
            num /= 10;
        }
    }

    result
}
