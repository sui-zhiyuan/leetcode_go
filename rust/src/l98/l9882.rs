pub fn count_pairs(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            if is_same(nums[i], nums[j]) {
                count += 1;
            }
        }
    }

    count
}

fn is_same(mut a: i32, mut b: i32) -> bool {
    let mut mis_match = Vec::new();

    while a > 0 || b > 0 {
        let da = a % 10;
        let db = b % 10;

        if da != db {
            mis_match.push((da , db));
        }

        a /= 10;
        b /= 10;
    }

    if mis_match.is_empty() {
        return true;
    }

    mis_match.len() == 2 && mis_match[0].0 == mis_match[1].1 && mis_match[0].1 == mis_match[1].0
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let nums = vec![3,12,30,17,21];
        assert_eq!(count_pairs(nums), 3);
    }
}