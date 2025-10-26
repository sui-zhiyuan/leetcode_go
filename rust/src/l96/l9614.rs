use std::collections::HashMap;

pub fn num_good_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as i64;

    let mut prefix_mod = HashMap::<i64, i64>::new();
    let mut result = 0i64;
    let mut curr_mod = 0i64;

    let mut repeat_value = -1i32;
    let mut repeat_count = 0i64;
    let mut repeat_mods = Vec::new();
    prefix_mod.insert(0, 1);
    for n in nums {
        curr_mod += n as i64;
        curr_mod %= k;

        result += prefix_mod.get(&curr_mod).unwrap_or(&0i64);
        *prefix_mod.entry(curr_mod).or_default() += 1;

        if n == repeat_value {
            repeat_count += 1;
            if (repeat_value as i64 * repeat_count) % k == 0 {
                repeat_mods.push(repeat_count);
            }

            continue;
        }

        for &v in repeat_mods.iter() {
            result -= repeat_count - v
        }
        repeat_value = n;
        repeat_count = 1;
        repeat_mods.clear();
        if (repeat_value as i64 * repeat_count) % k == 0 {
            repeat_mods.push(repeat_count);
        }
    }

    for &v in repeat_mods.iter() {
        result -= repeat_count - v
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(num_good_subarrays(vec![1, 2, 3], 3), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(num_good_subarrays(vec![2, 2, 2, 2, 2, 2], 6), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(num_good_subarrays(vec![100,100,100], 100), 3);
    }

    #[test]
    fn test4() {
        assert_eq!(num_good_subarrays(vec![1,1,2], 1), 5);
    }
}
