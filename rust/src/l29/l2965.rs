pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let n = grid.len();
    let pre_sum = (1..=(n*n)).fold(0, |acc, x| acc ^ x as u32);

    let mut sum = pre_sum;
    for row in grid.iter() {
        for v in row.iter() {
            sum ^= *v as u32;
        }
    }

    assert!(sum != 0);
    let filter = sum & !(sum - 1);

    let mut a_sum = 0;
    for i in 1..=(n*n){
        let i = i as u32;
        if i & filter != 0 {
            a_sum ^= i;
        }
    }

    for row in grid.iter() {
        for v in row.iter() {
            if (*v as u32) & filter != 0 {
                a_sum ^= *v as u32;
            }
        }
    }

    let mut result = vec![a_sum as i32, (a_sum ^ sum) as i32];

    for row in grid.iter() {
        for v in row.iter() {
            if *v == result[1] {
                result = vec![result[1], result[0]];
                break;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let grid = vec![
            vec![1, 2, 3, 3],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        let res = find_missing_and_repeated_values(grid);
        assert_eq!(res, vec![3, 4]);
    }
}