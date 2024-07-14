pub fn can_sort_array(nums: Vec<i32>) -> bool {
    let mut prev_max = 0;

    let mut curr_max = nums[0];
    let mut curr_min = nums[0];
    let mut prev_count_one = nums[0].count_ones();
    for v in nums.into_iter().skip(1) {
        if v.count_ones() == prev_count_one {
            curr_max = curr_max.max(v);
            curr_min = curr_min.min(v);
            continue;
        }

        if curr_min < prev_max {
            return false;
        }

        prev_max = curr_max;
        curr_max = v;
        curr_min = v;

        prev_count_one = v.count_ones();

    }

    curr_min > prev_max
}