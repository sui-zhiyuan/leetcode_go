pub fn wiggle_sort(nums: &mut[i32]) {
    let mut less = true;

    for i in 0..nums.len() -1 {
        if less && nums[i] > nums[i +1] || !less && nums[i] < nums[i +1] {
            nums.swap(i, i +1);
        }
        less = !less;
    }
}