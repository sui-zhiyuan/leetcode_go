pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
    let max_or = nums.iter().copied().reduce(|acc, v| acc | v).expect("nums must has value in it");

    let search = 2usize.pow(nums.len() as u32);

    let mut result = 0;
    for i in 1..search {
        let mut curr_or = 0;

        for (j, &v) in nums.iter().enumerate() {
            if 2usize.pow(j as u32) & i > 0 {
                curr_or |= v;
            }
        }

        if curr_or == max_or {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        assert_eq!(2 , count_max_or_subsets(vec![3,1]))
    }
}