use std::collections::BTreeMap;

impl Solution {
    pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        let mut values = changed
            .iter()
            .copied()
            .fold(BTreeMap::<i32, u32>::new(), |mut acc, v| {
                *acc.entry(v).or_insert(0) += 1;
                acc
            });

        let mut result = Vec::<i32>::new();
        while let Some((&key, &value)) = values.first_key_value() {
            if value == 1 {
                values.remove(&key);
            } else {
                *values.get_mut(&key).unwrap() -= 1;
            }

            result.push(key);
            let v = match values.get_mut(&(key * 2)) {
                Some(v) => v,
                None => {
                    return vec![];
                }
            };

            if *v > 1 {
                *v -= 1;
                continue;
            }

            values.remove(&(key * 2));
        }

        result
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let changed = vec![1, 3, 4, 2, 6, 8];
        let res = vec![1, 3, 4];
        assert_eq!(Solution::find_original_array(changed), res);
        let changed = vec![6, 3, 0, 1];
        let res = vec![];
        assert_eq!(Solution::find_original_array(changed), res);
        let changed = vec![1];
        let res = vec![];
        assert_eq!(Solution::find_original_array(changed), res);
        let changed = vec![4, 1, 2, 2];
        let res = vec![1, 2];
        assert_eq!(Solution::find_original_array(changed), res);
    }
}
