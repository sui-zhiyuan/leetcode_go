pub fn generate_key(num1: i32, num2: i32, num3: i32) -> i32 {
    let mut result = 0;
    let mut curr = 1;
    for _ in 0..4{
        let next = 10 * curr;
        let v1 = num1 % next / curr ;
        let v2 = num2 % next / curr ;
        let  v3 = num3 % next / curr ;
        result += v1.min(v2).min(v3) * curr;
        curr *= 10;
    }

    result
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_generate_key(){
        assert_eq!(generate_key(1234, 4567, 7893), 1233);
    }
}