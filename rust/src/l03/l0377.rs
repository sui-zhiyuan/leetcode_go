pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
    let mut state = vec![0; target as usize + 1];
    state[0] = 1;
    for i in 1..=target{
        for & n in nums.iter(){
            if i >= n {
                state[i as usize] += state[(i - n) as usize];
            }
        }
    }
    state[target as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(combination_sum4(vec![1, 2, 3], 4), 7)
    }

    #[test]
    fn test2() {
        assert_eq!(combination_sum4(vec![9], 3), 0)
    }
}