pub fn max_operations(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut sum = None;


    for v in nums.windows(2).step_by(2){
        if sum.is_none() {
            sum = Some(v[0] + v[1]);
            count += 1;
            continue;
        }
        if sum.unwrap() == v[0] + v[1] {
            count += 1;
        } else {
           break;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1,1,1,1,1,1,2,1,1,2];
        assert_eq!(max_operations(nums), 3);
    }
}