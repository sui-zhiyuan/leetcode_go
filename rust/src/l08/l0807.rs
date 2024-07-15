pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
    let top_row = grid.iter().map(|row| *row.iter().max().unwrap()).collect::<Vec<_>>();
    let top_col = (0..grid[0].len()).map(|col| grid.iter().map(|row| row[col]).max().unwrap()).collect::<Vec<_>>();

    let mut result = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, &v) in row.iter().enumerate() {
            result += (top_row[i].min(top_col[j]) - v).max(0);
        }
    }

    result
}