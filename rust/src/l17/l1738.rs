pub fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut x_sum = vec![vec![0; matrix[0].len()]; matrix.len()];

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            let mut v = matrix[i][j];
            if j > 0 {
                v ^= x_sum[i][j - 1];
            }
            if i > 0 {
                v ^= x_sum[i - 1][j];
            }
            if i > 0 && j > 0 {
                v ^= x_sum[i - 1][j - 1];
            }
            x_sum[i][j] = v;
        }
    }

    let mut x_sum = x_sum.into_iter().flatten().collect::<Vec<i32>>();
    x_sum.sort();
    x_sum[x_sum.len() - k as usize]
}
