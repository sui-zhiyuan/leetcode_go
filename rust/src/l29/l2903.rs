pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
    let index_difference = index_difference as usize;
    for i in 0..nums.len(){
        for j in i+index_difference..nums.len() {
            if (nums[i]- nums[j]).abs() >= value_difference {
                return vec![i as i32, j as i32];
            }
        }
    }

    vec![-1, -1]
}