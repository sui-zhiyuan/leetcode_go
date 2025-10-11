use std::cmp::Ordering;

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();

    let mut result = Vec::new();
    for (i, &ni) in nums.iter().enumerate() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        let mut j = i + 1;
        let mut k = nums.len() - 1;
        while j < k {
            match (ni + nums[j] + nums[k]).cmp(&0i32) {
                Ordering::Less => {
                    j += 1;
                    continue;
                }
                Ordering::Greater => {
                    k -= 1;
                    continue;
                }
                Ordering::Equal => (),
            }

            result.push(vec![ni, nums[j], nums[k]]);

            let j_curr = j;
            while j < k && nums[j] == nums[j_curr] {
                j += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::parse_grid;
    use crate::parameter_tests;

    struct Case {
        nums: Vec<i32>,
        expect: &'static str,
    }

    fn test(c: Case) {
        let expect = parse_grid::<i32>(c.expect).unwrap();
        assert_eq!(expect, three_sum(c.nums))
    }

    parameter_tests! {
        test,
        (case1: Case {
            nums: vec![-1,0,1,2,-1,-4],
            expect: "[[-1,-1,2],[-1,0,1]]",
        }),
        (case2: Case {
            nums: vec![0,1,1],
            expect: "[]",
        }),
        (case3: Case {
            nums: vec![0,0,0],
            expect: "[[0,0,0]]",
        }),
    }
}
