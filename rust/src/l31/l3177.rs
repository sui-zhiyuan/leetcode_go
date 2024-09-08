use std::collections::HashMap;

pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut curr = vec![0;k+1];
    let mut next = vec![0;k+1];
    let mut max_end: HashMap<i32, Vec<i32>> = HashMap::new();
    for v in nums.iter(){
        for j in 0..=k {
            next[j] = max_end.get(v).map(|e| e[j]).unwrap_or(0) + 1;
            if j > 0 {
                next[j] = next[j].max(curr[j - 1] + 1);
            }
        }
        max_end.insert(*v, next.clone());
        for j in 0..=k {
            next[j] = next[j].max(curr[j]);
        }
        std::mem::swap(&mut curr, &mut next);
    }

    curr[k]
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let nums = vec![1,2,1,1,3];
        let k = 2;
        let result = 4;
        assert_eq!(maximum_length(nums, k), result);
    }

    #[test]
    fn test2(){
        let nums = vec![2,2,3,3];
        let k = 0;
        let result = 2;
        assert_eq!(maximum_length(nums, k), result);
    }

    #[test]
    fn test3(){
        let nums = vec![2,2,3];
        let k = 0;
        let result = 2;
        assert_eq!(maximum_length(nums, k), result);
    }

    #[test]
    fn test4(){
        let nums = vec![89,89,90,88,88,88,88,90,90];
        let k = 2;
        let result = 8;
        assert_eq!(maximum_length(nums, k), result);
    }
}