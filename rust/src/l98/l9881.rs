pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
    for i in 0..k{
        let i_min = nums.iter().enumerate().min_by_key(|v| v.1).unwrap().0;
        nums[i_min] *= multiplier;
    }

    nums
}