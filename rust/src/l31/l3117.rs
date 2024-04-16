impl Solution {
    pub fn minimum_value_sum(nums: Vec<i32>, and_values: Vec<i32>) -> i32 {
        Self::inner(&nums, &and_values)
    }

    pub fn inner(nums: &[i32], and_value: &[i32]) -> i32 {
        dbg!(nums, and_value);
        if nums.is_empty() {
            return -1;
        }
        if nums.len() < and_value.len() {
            return -1;
        }

        if and_value.len() == 1 {
            let all_and = nums.iter().copied().reduce(|acc, x| acc & x).unwrap();
            if and_value[0] == all_and {
                return *nums.last().unwrap();
            } else {
                return -1;
            }
        }

        let (mut left_start, mut left_end, mut right_stat, mut right_end) =
            (usize::MAX, usize::MAX, usize::MAX, usize::MAX);
        let mut acc = -1;
        for i in 0..nums.len() {
            if acc == -1 {
                acc = nums[i];
            } else {
                acc &= nums[i];
            }

            //dbg!(acc , and_value[0] , left_start , left_end);

            if acc == and_value[0] && left_start == usize::MAX {
                left_start = i;
            }

            if acc & and_value[0] != and_value[0] {
                left_end = i;
                break;
            }
        }
        let mut acc = -1;
        let end_and = and_value[and_value.len() - 1];
        for i in (0..nums.len()).rev() {
            if acc == -1 {
                acc = nums[i];
            } else {
                acc &= nums[i];
            }

            if acc == end_and && right_end == usize::MAX {
                right_end = i + 1;
            }
            // dbg!(i, acc , end_and , right_stat , right_end);
            if acc & end_and != end_and {
                right_stat = i + 1;
                break;
            }
        }

        if left_end == usize::MAX {
            left_end = nums.len();
        }
        if right_stat == usize::MAX {
            right_stat = 0;
        }

        if left_start == usize::MAX && right_end == usize::MAX {
            return -1;
        }

        let mut use_left = true;

        if left_start != usize::MAX && right_end != usize::MAX {
            let left_len = left_end - left_start;
            let right_len = right_end - right_stat;
            use_left = left_len < right_len
        } else if left_start != usize::MAX {
            use_left = true;
        } else if right_end != usize::MAX {
            use_left = false;
        }
        if use_left {
            dbg!("left" ,&nums[left_start..left_end], and_value[0]);
            let mut ans = i32::MAX;
            for i in left_start..left_end {
                let sub= Self::inner(&nums[i + 1..], &and_value[1..]);
                if sub >= 0 && sub + nums[i] < ans {
                    ans = sub + nums[i];
                }
            }
            dbg!(ans);
            ans
        } else {
            dbg!("right" ,&nums[right_stat..right_end], and_value[and_value.len() - 1]);
            let mut ans = i32::MAX;
            for i in right_stat..right_end {
                let sub = Self::inner(&nums[..i], &and_value[..and_value.len() - 1]);
                if sub != -1 && sub + nums.last().unwrap()  < ans {
                    ans = sub + nums.last().unwrap();
                }
            }
            dbg!(ans);
            ans
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::minimum_value_sum(vec![1, 4, 3, 3, 2], vec![0, 3, 3, 2]),
            12
        );
    }
}
