pub fn temperature_trend(temperature_a: Vec<i32>, temperature_b: Vec<i32>) -> i32 {
    let temp_a = temperature_a.windows(2).map(|v| v[0].cmp(&v[1]));
    let temp_b = temperature_b.windows(2).map(|v| v[0].cmp(&v[1]));

    let ct = temp_a.zip(temp_b).map(|(a, b)| a == b);

    ct.scan(0, | state , v| {
        if v {
            *state +=1; 
        } else {
            *state = 0
        };
        Some(*state)
    }).max().unwrap()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let temperature_a = vec![-14,7,-19,9,13,40,19,15,-18];
        let temperature_b = vec![3,16,28,32,25,12,13,-6,4];
        assert_eq!(temperature_trend(temperature_a, temperature_b) , 1)
    }
}