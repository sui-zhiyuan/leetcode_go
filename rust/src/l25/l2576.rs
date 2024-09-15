pub fn max_num_of_marked_indices(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();

    let mid = nums.len() / 2;
    let mut curr = mid;

    let mut result = 0;
    for v in nums[0..mid].iter() {
        while curr < nums.len() && nums[curr] < v * 2 {
            curr += 1;
        }

        if curr >= nums.len() {
            break;
        }
        result += 1;
        curr += 1;
    }

    result * 2
}
