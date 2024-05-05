pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums = nums.into_iter().enumerate().collect::<Vec<_>>();
    nums.sort_by_key(|v| v.1);

    let (mut left, mut right) = (0, nums.len() - 1);
    while left < right {
        let sum = nums[left].1 + nums[right].1;
        match sum.cmp(&target) {
            std::cmp::Ordering::Equal => return vec![nums[left].0 as i32, nums[right].0 as i32],
            std::cmp::Ordering::Less => left += 1,
            std::cmp::Ordering::Greater => right -= 1,
        }
    }

    unreachable!("No solution found")
}
