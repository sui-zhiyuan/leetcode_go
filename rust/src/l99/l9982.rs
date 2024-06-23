pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {

    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut min_y = usize::MAX;
    let mut max_y = usize::MIN;

    for (i , row) in grid.iter().enumerate(){
        for (j, &cell) in row.iter().enumerate(){
            if cell == 1{
                min_x = min_x.min(i);
                max_x = max_x.max(i);
                min_y = min_y.min(j);
                max_y = max_y.max(j);
            }
        }
    }

    ((max_x - min_x + 1) * (max_y - min_y + 1)) as i32
}