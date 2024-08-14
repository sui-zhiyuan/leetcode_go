pub fn is_array_special(nums: Vec<i32>) -> bool {
    let nums = nums.into_iter().map(|x| x & 0x01).collect::<Vec<i32>>();

    nums.windows(2).all(|w| w[0] + w[1] == 1)
}