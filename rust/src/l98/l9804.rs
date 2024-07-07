// cspell:ignore subarrays

pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as u32;
    let nums = nums.into_iter().map(|v| v as u32).map(|v| if v & k >= k {v & !k} else {u32::MAX}).collect::<Vec<_>>();

    let mut start = usize::MAX;
    let mut result = 0;
    for (i , &curr) in nums.iter().enumerate(){
        if curr == u32::MAX {
            if i > start {
                result += count_and_zero(&nums[start..i]);
            }
            start = usize::MAX;
            continue;
        }

        if start == usize::MAX {
            start = i;
        }
    }
    if start != usize::MAX {
        result += count_and_zero(&nums[start..]);
    }
    
    result
}

fn count_and_zero(nums: &[u32]) -> i64 {
    let mut count_zero = vec![[0; 32]; nums.len()];
    for (i , &v) in nums.iter().enumerate() {
        for j in 0..32{
            let prev = if i > 0 {count_zero[i-1][j]} else {0};
            count_zero[i][j] = prev + if v & (1 << j) == 0 {1} else {0};
        }
    }
    let len = nums.len();
    let mut result = 0;

    for (start , _ ) in nums.iter().enumerate(){
        let end = binary_search(start, nums.len(), |end| {
            for j in 0..32 {
                let start_v = if start > 0 {count_zero[start-1][j]} else {0};
                if count_zero[end][j] - start_v == 0 {
                    return false;
                }
            }
            true
        });

        result += (len -end) as i64;
    }

    result
}

fn binary_search(mut left: usize, mut right:usize, mut check: impl FnMut(usize) -> bool) -> usize
{
    while left < right - 1 {
        let mid = (left + right) / 2;
        if check(mid) {
            right = mid;
        } else {
            left = mid;
        }
    }
    
    if check(left) {
        left
    } else {
        right
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let nums = vec![1,1,1];
        let k = 1;
        assert_eq!(count_subarrays(nums, k), 6);
    }
}