pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let sum_values = nums
        .iter()
        .scan(0, |state, &v| {
            *state += v;
            Some(*state)
        })
        .collect::<Vec<i32>>();

    let sum = sum_values.last().unwrap();
    for i in 0..nums.len(){
        let left = sum_values[i] - nums[i];
        let right = sum - sum_values[i];
        if left == right {
            return i as i32;
        }
    }

    -1
}
