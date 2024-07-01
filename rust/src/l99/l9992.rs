pub fn maximum_length(nums: Vec<i32>) -> i32 {
    let nums = nums.into_iter().map(|x| x % 2 == 0).collect::<Vec<_>>();
    let count0 = nums.iter().filter(|&&x| x).count() as i32;
    let count2 = nums.iter().filter(|&&x| !x).count() as i32;

    let mut count1 = 0;
    let mut prev = !nums[0];

    for v in nums {
        if v == prev{
            continue;
        }

        count1 += 1;
        prev = v;
    }

    count0.max(count1.max(count2))
}