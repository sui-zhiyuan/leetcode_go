pub fn maximum_sum(arr: Vec<i32>) -> i32 {
    let mut state = vec![(0 , 0); arr.len() + 1];
    state[0] = (0 , 0);
    let mut max = 0;
    for (i , &v ) in arr.iter().enumerate(){
        let max_0 = (state[i].0 + v).max(v);
        let max_1 = (state[i].1 + v).max(state[i].0);
        let max = max.max(max_0).max(max_1);
        state[i + 1] = (max_0 , max_1);
    }
    
    max
}
