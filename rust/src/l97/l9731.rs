use std::collections::HashMap;

pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let k = k as usize;
    let x = x as usize;
    let mut count = HashMap::<i32 , i32>::new();

    for &v in nums[0..k-1].iter(){
        *count.entry(v).or_insert(0) += 1;
    }

    let mut res = Vec::<i32>::new();
    for i in k-1..nums.len(){
        *count.entry(nums[i]).or_insert(0) += 1;
        let mut count_list: Vec<(i32, i32)> = count.iter().map(|(&value , &count)| (value, count)).collect();
        count_list.sort_by(|a, b| a.1.cmp(&b.1).then_with(||a.0.cmp(&b.0)).reverse());
        res.push(count_list[..x.min(count_list.len())].iter().map(|v| v.0 * v.1).sum());
        let start = i + 1 - k ;
        *count.get_mut(&nums[start]).unwrap() -= 1;
    }

    res
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let nums = vec![1,1,2,2,3,4,2,3];
        let k = 6;
        let x = 2;
        let res = find_x_sum(nums, k, x);
        assert_eq!(res, vec![6, 10, 12]);
    }

    #[test]
    fn test2(){
        let nums = vec![9,2,2];
        let k = 3;
        let x = 3;
        let res = find_x_sum(nums, k, x);
        assert_eq!(res, vec![13]);
    }
}