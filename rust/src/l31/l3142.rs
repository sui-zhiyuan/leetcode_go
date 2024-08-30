pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
    let n = grid.len();
    let m = grid[0].len();
    for (i , row) in grid.iter().enumerate(){
        for (j, &v) in row.iter().enumerate(){
            if i +1 < n && grid[i+1][j] != v{
                return false;
            }
            if j + 1 < m && grid[i][j+1] == v{
                return false;
            }
        }
    }

    true
}