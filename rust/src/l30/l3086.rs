pub fn minimum_moves(nums: Vec<i32>, k: i32, max_changes: i32) -> i64 {
    let mut result = i64::MAX;
    if max_changes >= k {
        result = result.min((k * 2) as i64);
    }

    if k >= 1 && max_changes >= k - 1 && nums.iter().copied().any(|v| v == 1) {
        result = result.min(((k - 1) * 2) as i64);
    }

    if k >= 2 && max_changes >= k - 2 && nums.windows(2).any(|v| v == [1, 1]) {
        result = result.min(((k - 2) * 2) as i64 + 1);
    }

    if k >= 3 && max_changes >= k - 3 && nums.windows(3).any(|v| v == [1, 1, 1]) {
        result = result.min(((k - 3) * 2) as i64 + 2);
    }

    if let Some(v) = inner(&nums, k - max_changes) {
        result = result.min(v + 2 * max_changes as i64);
    }

    result
}

fn inner(nums: &[i32], k: i32) -> Option<i64> {
    if k <= 0 {
        return Some(0);
    }
    let mut start = 0;
    let mut end = 0;
    let mut mid = 0;
    let mut count = nums[end];
    let mut left_count = nums[end];
    let mut cost = 0;

    while count < k {
        end += 1;
        while end < nums.len() && nums[end] == 0 {
            end += 1;
        }
        if end == nums.len() {
            break;
        }
        count += nums[end];
        cost += (end - mid) as i64;
        let original_mid = mid;
        while left_count < (count + 1) / 2 {
            mid += 1;
            left_count += nums[mid];
        }
        cost += ((2 * left_count - 2 - count) * (mid - original_mid) as i32) as i64;
    }

    while nums[start] == 0 {
        start += 1;
        
    }

    if count < k {
        return None;
    }

    let mut min_cost = cost;

    while end < nums.len() {
        end += 1;
        while end < nums.len() && nums[end] == 0 {
            end += 1;
        }
        if end == nums.len() {
            break;
        }
        cost += (end - mid) as i64;
        cost -= (mid - start) as i64;
        start += 1;
        while nums[start] == 0 {
            start += 1;
        }

        let original_mid = mid;
        mid += 1;
        while nums[mid] == 0 {
            mid += 1;
        }
        cost += ((2 * left_count - 2 - count) * (mid - original_mid) as i32) as i64;
        min_cost = min_cost.min(cost);
    }

    Some(min_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(minimum_moves(vec![1, 1, 0, 0, 0, 1, 1, 0, 0, 1], 3, 1), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(minimum_moves(vec![0, 0, 0, 0], 2, 3), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(minimum_moves(vec![1,1], 1, 2), 0);
    }

    #[test]
    fn test_4() {
        assert_eq!(minimum_moves(vec![0,0,1,1], 1, 0), 0);
    }
}
