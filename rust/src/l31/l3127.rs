pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
    let n = grid.len();
    let m = grid[0].len();
    for (i, row) in grid[..n - 1].iter().enumerate() {
        for (j, _) in row[..m - 1].iter().enumerate() {
            let mut count_w = 0;

            if grid[i][j] == 'W' {
                count_w += 1;
            }
            if grid[i + 1][j] == 'W' {
                count_w += 1;
            }
            if grid[i][j + 1] == 'W' {
                count_w += 1;
            }
            if grid[i + 1][j + 1] == 'W' {
                count_w += 1;
            }

            if count_w != 2 {
                return true;
            }
        }
    }

    false
}
