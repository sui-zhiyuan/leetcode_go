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

    if mis_match.len() == 2 {
        let (a1 , b1) = mis_match[0];
        let (a2 , b2) = mis_match[1];
        return a1 == b2 && a2 == b1;
    }

    if mis_match.len() == 3 {
        let (a1 , b1) = mis_match[0];
        let (a2 , b2) = mis_match[1];
        let (a3 , b3) = mis_match[2];
        return (a1 == b2 && a2 == b3 && a3==b1 ) || (a1 == b3 && a3 == b2 && a2==b1); 
    }

    if mis_match.len() == 4 {
        let (a1 , b1) = mis_match[0];
        let (a2 , b2) = mis_match[1];
        let (a3 , b3) = mis_match[2];
        let (a4 , b4) = mis_match[3];

        return (a1 == b2 && a2 == b1 && a3 == b4 && a4 == b3) 
            || (a1 == b3 && a3 == b1 && a2 == b4 && a4 == b2) 
            || (a1 == b4 && a4 == b1 && a2 == b3 && a3 == b2);
    }

    false
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let nums = vec![1023 , 2310];
        assert_eq!(count_pairs(nums), 1);
    }
}