use std::collections::HashMap;

pub fn count_stable_subarrays(capacity: Vec<i32>) -> i64 {
    // HashMap<prefix_sum , <Hashmap<last_value, count>>
    let mut prefix_sums = HashMap::<i64, HashMap<i32, i64>>::new();

    let mut prefix_sum = 0i64;
    let mut result = 0;
    for (i, &v) in capacity.iter().enumerate() {
        if let Some(maps) = prefix_sums.get(&(prefix_sum - v as i64)) {
            result += maps.get(&v).unwrap_or(&0);
            if i >= 1 && capacity[i] == 0 && capacity[i - 1] == 0 {
                result -= 1;
            }
        }

        // let sumn = i + 1;
        prefix_sum += v as i64;

        let t = prefix_sums.entry(prefix_sum).or_default();
        *t.entry(capacity[i]).or_default() += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(2, count_stable_subarrays(vec![9, 3, 3, 3, 9]));
    }

    #[test]
    fn test2() {
        assert_eq!(0, count_stable_subarrays(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test3() {
        assert_eq!(1, count_stable_subarrays(vec![-4, 4, 0, 0, -8, -4]));
    }
}
