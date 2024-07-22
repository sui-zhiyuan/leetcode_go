pub fn maximum_sum(arr: Vec<i32>) -> i32 {
    let mut state = vec![(0 , 0); arr.len()];
    state[0] = (arr[0] , arr[0]);
    let mut max = arr[0];
    for (i , &v ) in arr.iter().enumerate().skip(1){
        let max_0 = (state[i-1].0 + v).max(v);
        let max_1 = (state[i-1].1 + v).max(state[i-1].0);
        max = max.max(max_0).max(max_1);
        state[i] = (max_0 , max_1);
    }
    
    max
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let arr = vec![1, -2, 0, 3];
        assert_eq!(maximum_sum(arr), 4);
    }
}