pub fn maximum_total_cost(nums: Vec<i32>) -> i64 {
    let mut cost = vec![0i64; nums.len()];

    for (i , _) in nums.iter().enumerate(){
        let mut max_cost =  i64::MIN;
        if i == 0 {
            max_cost = max_cost.max(nums[i] as i64);
        }
        if i == 1 {
            max_cost = max_cost.max(nums[0] as i64 - nums[1] as i64);
        }
        if i >=2 {
            max_cost = max_cost.max(cost[i-2] + nums[i-1] as i64 - nums[i] as i64);
        }
        if i >=1 {
            max_cost = max_cost.max(cost[i-1]+ nums[i] as i64);
        }
        cost[i] = max_cost;
    }

    cost[nums.len()-1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![4, 2, 1, 3];
        let res = 10;
        assert_eq!(maximum_total_cost(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![1,-2,3,4];
        let res = 10;
        assert_eq!(maximum_total_cost(nums), res);
    }

    #[test]
    fn test3() {
        let nums = vec![1,-1,1,-1];
        let res = 4;
        assert_eq!(maximum_total_cost(nums), res);
    }

    #[test]
    fn test4() {
        let nums = vec![0];
        let res = 0;
        assert_eq!(maximum_total_cost(nums), res);
    }

    #[test]
    fn test5() {
        let nums = vec![1,-1];
        let res = 2;
        assert_eq!(maximum_total_cost(nums), res);
    }

    #[test]
    fn test6() {
        let nums = vec![-1,-2,-3,-4];
        let res = 2;
        assert_eq!(maximum_total_cost(nums), res);
    }
}