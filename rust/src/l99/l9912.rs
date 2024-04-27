pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
    let mut row_count = vec![0; grid.len()];
    let mut col_count = vec![0; grid[0].len()];

    for (i , row) in grid.iter().enumerate(){
        for (j , &cell) in row.iter().enumerate() {
            if cell == 0 {
                continue;
            }

            row_count[i] += 1;
            col_count[j] += 1;
        }
    }

    let mut res : i64 = 0;
    for (i , row) in grid.iter().enumerate(){
        for (j , &cell) in row.iter().enumerate() {
            if cell == 0 {
                continue;
            }

            res += (row_count[i] - 1) * (col_count[j] - 1);
        }
    }


    res
}