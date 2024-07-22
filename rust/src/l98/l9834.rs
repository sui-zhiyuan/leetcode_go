pub fn minimum_operations(nums: Vec<i32>, target: Vec<i32>) -> i64 {
    let mut result = 0;
    let mut change: i32 = 0;
    for v in nums.into_iter().zip(target).map(|(n , t)| n - t){
        if change.signum() * v.signum() == 1{
            if change.abs() <= v.abs(){
                result += (v.abs() - change.abs()) as i64;
            }

            change = v;
            continue;
        }

        change = v;
        result += v.abs() as i64;
    }

    result
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let nums = vec![3,5,1,2];
        let target = vec![4,6,2,4];
        assert_eq!(minimum_operations(nums, target), 2);
    }

    #[test]
    fn test3(){
        let nums = vec![1, 2, 3, 5, 6];
        let target = vec![1, 3, 3, 4, 6];
        assert_eq!(minimum_operations(nums, target), 1);
    }
}