pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {
    let p0 = Point{x: 0, y: 0};
    let p1 = Point{x: grid.len()-1, y: grid[0].len()-1};

    let mut min_value = i32::MAX;
    for row in p0.x .. p1.x{
        let s1 = size(&grid, Point{x: p0.x, y: p0.y}, Point{x: row, y: p1.y});
        let s2 = max_div_2(&grid, Point{x: row+1, y: p0.y}, Point{x: p1.x, y: p1.y});
        min_value = min_value.min(s1 + s2);

        let s1 = max_div_2(&grid, Point{x: p0.x, y: p0.y}, Point{x: row, y: p1.y});
        let s2 = size(&grid, Point{x: row+1, y: p0.y}, Point{x: p1.x, y: p1.y});
        min_value = min_value.min(s1 + s2);
    }

    for col in p0.y .. p1.y{
        let s1 = size(&grid, Point{x: p0.x, y: p0.y}, Point{x: p1.x, y: col});
        let s2 = max_div_2(&grid, Point{x: p0.x, y: col+1}, Point{x: p1.x, y: p1.y});
        min_value = min_value.min(s1 + s2);

        let s1 = max_div_2(&grid, Point{x: p0.x, y: p0.y}, Point{x: p1.x, y: col});
        let s2 = size(&grid, Point{x: p0.x, y: col+1}, Point{x: p1.x, y: p1.y});
        min_value = min_value.min(s1 + s2);
    }

    min_value
}

fn max_div_2(grid: &[Vec<i32>] , p0 : Point , p1: Point) -> i32 {
    let mut min_value = i32::MAX;

    for row in p0.x .. p1.x{
        let s1 = size(grid, Point{x: p0.x, y: p0.y}, Point{x: row, y: p1.y});
        let s2 = size(grid, Point{x: row+1, y: p0.y}, Point{x: p1.x, y: p1.y});
        min_value = min_value.min(s1 + s2);
    }

    for col in p0.y .. p1.y{
        let s1 = size(grid, Point{x: p0.x, y: p0.y}, Point{x: p1.x, y: col});
        let s2 = size(grid, Point{x: p0.x, y: col+1}, Point{x: p1.x, y: p1.y});
        min_value = min_value.min(s1 + s2);
    }

    min_value
}

fn size(grid: &[Vec<i32>] , p0: Point , p1: Point) -> i32 {
    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut min_y = usize::MAX;
    let mut max_y = usize::MIN;

    let mut find = false;
    for (row , v_row) in grid.iter().enumerate().take(p1.x + 1).skip(p0.x) {
        for (col , &v_cell) in v_row.iter().enumerate().take(p1.y + 1).skip(p0.y){
            if v_cell == 1 {
                find = true;
                min_x = min_x.min(row);
                max_x = max_x.max(row);
                min_y = min_y.min(col);
                max_y = max_y.max(col);
            }
        }
    }
    if !find {
        return 0;
    }
    ((max_x - min_x + 1) * (max_y - min_y + 1)) as i32
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod tests {
    use crate::common;

    use super::*;

    #[test]
    fn test1() {
        let grid = common::parse_grid::<i32>("[[1,0,1],[1,1,1]]").unwrap();
        let res = 5;
        assert_eq!(minimum_sum(grid), res);
    }

    #[test]
    fn test2() {
        let grid = common::parse_grid::<i32>("[[1,0,1,0],[0,1,0,1]]").unwrap();
        let res = 5;
        assert_eq!(minimum_sum(grid), res);
    }
}