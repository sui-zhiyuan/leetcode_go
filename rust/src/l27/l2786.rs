pub fn max_score(nums: Vec<i32>, x: i32) -> i64 {
    let x = x as i64;
    let mut last = [Option::<i64>::None, Option::<i64>::None];
    last[(nums[0] % 2) as usize] = Some(nums[0] as i64);
    let mut max = nums[0] as i64;

    for v in nums.into_iter().skip(1) {
        let mut curr = i64::MIN;
        let v = v as i64;
        let parity = (v % 2) as usize;
        if let Some(l) = last[parity] {
            curr = curr.max(l + v);
        }
        if let Some(l) = last[1 - parity] {
            curr = curr.max(l + v - x);
        }
        max = max.max(curr);
        last[parity] = Some(curr);
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![8, 50, 65, 85, 8, 73, 55, 50, 29, 95, 5, 68, 52, 79];
        let x = 74;
        assert_eq!(max_score(nums, x), 470);
    }

    #[test]
    fn test_2() {
        let nums = vec![9,58,17,54,91,90,32,6,13,67,24,80,8,56,29,66,85,38,45,13,20,73,16,98,28,56,23,2,47,85,11,97,72,2,28,52,33];
        let x = 90;
        assert_eq!(max_score(nums, x), 886);
    }
}
