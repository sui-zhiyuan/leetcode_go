pub fn count_k_constraint_substrings(s: String, k: i32, queries: Vec<Vec<i32>>) -> Vec<i64> {
    let s = s.chars().collect::<Vec<_>>();
    let k = k as usize;
    let n = s.len();
    let mut total = vec![State{count_0:0 , count_1:0}; n+1];

    for (i , &c) in s.iter().enumerate(){
        total[i+1] = State{
            count_0: total[i].count_0 + (c == '0') as usize,
            count_1: total[i].count_1 + (c == '1') as usize,
        };
    }

    let mut start_pos = vec![0; n+1];
    let mut start = 0;
    for i in 1..=n{
        while total[i].count_0 - total[start].count_0 > k || total[i].count_1 - total[start].count_1 > k{
            start += 1;
        }
        start_pos[i] = start;
    }

    let mut sum_all = vec![0i64 ; n+1];
    for i in 1..=n{
        sum_all[i] = sum_all[i-1] + (i- start_pos[i]) as i64;
    }
    let mut results = Vec::new();
    for q in queries{
        let start = q[0] as usize;
        let end = q[1] as usize +1;

        let mut result = sum_all[end] - sum_all[start];
        for j in start..end {
            if start_pos[j] >= start {
                break;
            }
            result -= (j - start_pos[j] -1) as i64;
            result += (j-start -1) as i64;
        }
        results.push(result);
    }
    results
}

#[derive(Debug, Clone, Copy)]
struct State{
    count_0: usize,
    count_1: usize,
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_count_k_constraint_substrings(){
        let s = "0001111".to_string();
        let k = 2;
        let queries = [[0,6]];
        let queries = queries.iter().map(|x| x.to_vec()).collect();
        let result = count_k_constraint_substrings(s, k, queries);
        assert_eq!(result, vec![26]);
    }
}