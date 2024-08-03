pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
    let n = grid.len();
    let m = grid[0].len();

    let mut count_row = vec![0; n];
    let mut count_col = vec![0; m];
    for ( i , row ) in grid.iter().enumerate() {
        for ( j , &cell ) in row.iter().enumerate() {
            count_row[i] += cell;
            count_col[j] += cell;
        }
    }

    let mut result = 0;
    for ( i , row ) in grid.iter().enumerate() {
        for ( j , &cell ) in row.iter().enumerate() {
            if cell == 0 {
                continue;
            }
            result += (count_row[i] - 1) as i64 * (count_col[j] - 1) as i64;
        }
    }
    result
}