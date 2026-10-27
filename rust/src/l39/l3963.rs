pub fn create_grid(m: i32, n: i32) -> Vec<String> {
    let m = m as usize;
    let n = n as usize;
    let mut result = vec![vec![false; n]; m];

    for i in 0..n {
        result[0][i] = true;
    }

    for i in 0..m {
        result[i][n - 1] = true;
    }

    result
        .into_iter()
        .map(|row| {
            let r: String = row.into_iter().map(|v| if v { '.' } else { '#' }).collect();
            r
        })
        .collect()
}
