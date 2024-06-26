pub fn good_subsetof_binary_matrix(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let values = grid
        .iter()
        .map(|row| row.iter().fold(0, |acc, &v| acc * 2 + (v as u32)))
        .collect::<Vec<_>>();

    if let Some(e) = values.iter().position(|&v| v == 0) {
        return vec![e as i32];
    }

    for (i , v) in values.iter().enumerate() {
        for (j, w) in values.iter().enumerate().skip(i + 1) {
            if v & w == 0 {
                return vec![i as i32, j as i32];
            }
        }
    }

    vec![]
}
