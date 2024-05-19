pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    if nums.is_empty() {
        return vec![true ; queries.len()];
    }

    let is_even = nums.iter().map(|&v| v % 2 == 0).collect::<Vec<bool>>();

    let mut end = vec![0 ; nums.len()];
    for i in 0..nums.len() {
        if i >0 && end[i-1] > i {
            end[i] = end[i-1];
            continue;
        }
        let mut j = i+1;
        while j < nums.len() && is_even[j] != is_even[j-1] {
            j += 1;
        }
        end[i]= j-1;
    }

    // dbg!(&end);
    queries.iter().map(|v| end[v[0] as usize] >= v[1] as usize).collect()
}

#[cfg(test)]
mod tests {
    use crate::common;

    use super::*;

    #[test]
    fn test1() {
        let nums = vec![3,4,1,2,6];
        let queries = vec![vec![0,4]];
        let res = vec![false];
        assert_eq!(is_array_special(nums, queries), res);
    }

    #[test]
    fn test2() {
        let nums = vec![4,3,1,6];
        let queries = common::parse_grid::<i32>("[[0,2],[2,3]]").unwrap();
        let res = vec![false ,true];
        assert_eq!(is_array_special(nums, queries), res);
    }
}