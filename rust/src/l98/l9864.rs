use std::mem;

pub fn maximum_value_sum(board: Vec<Vec<i32>>) -> i64 {
    let n = board[0].len();

    let mut curr_state0 = vec![None; n];
    let mut curr_state1 = vec![None; n*n];

    let mut next_state0 = vec![None; n];
    let mut next_state1 = vec![None; n*n];

    let mut result = i64::MIN;


    for row in board.iter(){
        let mut max_values = [(i64::MIN, 0); 3];
        for (i , &v1) in row.iter().enumerate(){
            let v1= v1 as i64;
            if v1 > max_values[0].0{
                max_values[2] = max_values[1];
                max_values[1] = max_values[0];
                max_values[0] = (v1, i);
            }
            else if v1 > max_values[1].0{
                max_values[2] = max_values[1];
                max_values[1] = (v1, i);
            }
            else if v1 > max_values[2].0{
                max_values[2] = (v1, i);
            }

            next_state0[i] = max(curr_state0[i], Some(v1));
            for (j , &v2) in row.iter().enumerate(){
                if i == j {
                    continue;
                }

                let index = i*n + j;
                next_state1[index] = max(curr_state1[index], add(curr_state0[i], Some(v2 as i64)));
            }
        }

        for (index , &v) in curr_state1.iter().enumerate(){
            let Some(v) = v else {
                continue;
            };
            let i = index / n;
            let j = index % n;
            if i == max_values[0].1 && j == max_values[1].1 || i == max_values[1].1 && j == max_values[0].1{
                result = result.max(v  + max_values[2].0);
            }else if i == max_values[0].1 || j == max_values[0].1{
                result = result.max(v  + max_values[1].0);
            }else {
                result = result.max(v  + max_values[0].0);
            }
        }

        mem::swap(&mut curr_state0, &mut next_state0);
        mem::swap(&mut curr_state1, &mut next_state1);
        next_state0.fill(None);
        next_state1.fill(None);
    }

    result
}

fn max(a: Option<i64> , b: Option<i64>) -> Option<i64>{
    match (a,b){
        (Some(a), Some(b)) => Some(a.max(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

fn add(a: Option<i64> , b: Option<i64>) -> Option<i64>{
    match (a,b){
        (Some(a), Some(b)) => Some(a + b),
        (Some(_), None) => None,
        (None, Some(_)) => None,
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_maximum_value_sum(){
        let board = [[83,-5,-11],[-4,-82,51],[-37,-36,-84]];
        let board = board.iter().map(|row| row.to_vec()).collect();
        assert_eq!(maximum_value_sum(board), 98);
    }
}