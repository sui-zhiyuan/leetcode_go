use std::cmp;

pub fn minimum_cost(_m: i32, _n: i32, mut horizontal_cut: Vec<i32>, mut vertical_cut: Vec<i32>) -> i32 {
    let m = horizontal_cut.len() +1;
    let n = vertical_cut.len() +1;
    horizontal_cut.sort_by_key(|v| cmp::Reverse(*v));
    let mut horizontal_cut1 = vec![0];
    horizontal_cut1.append(&mut horizontal_cut);
    let mut horizontal_cut = horizontal_cut1;
    vertical_cut.sort_by_key(|v| cmp::Reverse(*v));
    let mut vertical_cut1 = vec![0];
    vertical_cut1.append(&mut vertical_cut);
    let mut vertical_cut = vertical_cut1;
    let mut curr = &mut vertical_cut.iter().scan(0 , | s , &v| {
        *s += v as i64;
        Some(*s)
    }).collect::<Vec<_>>();
    let mut next = &mut vec![0 ; n];

    for row in 1..m{
        next[0] = curr[0] + horizontal_cut[row] as i64;
        for col in 1..n{
            let row_prev = next[col-1];
            let v_cost = vertical_cut[col];
            let v1 = row_prev + v_cost as i64 * (row as i64+1);

            let col_prev = curr[col];
            let h_cost = horizontal_cut[row];
            let v2 = col_prev + h_cost as i64 * (col as i64+1);
            next[col] = v1.min(v2);
        }
        (curr , next) = (next , curr)
    }

    curr[n-1] as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let m = 5;
        let n = 4;
        let horizontal_cut = vec![1,3];
        let vertical_cut = vec![5];
        let res = 13;
        assert_eq!(minimum_cost(m, n, horizontal_cut, vertical_cut), res);
    }

}