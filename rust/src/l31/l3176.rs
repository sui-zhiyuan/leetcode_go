pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut state = vec![vec![1;k+1]; nums.len()];
    for (i , vi) in nums.iter().enumerate().skip(1){
        for (j  ,vj) in nums.iter().enumerate().take(i){
            for l in 0..=k{
                if vi == vj {
                    state[i][l] = state[i][l].max(state[j][l] + 1);
                } else if l > 0 {
                    state[i][l] = state[i][l].max(state[j][l - 1] + 1);
                }
            }
        }
    }

    state.iter().map(|x| x[k]).max().unwrap() as i32
}
