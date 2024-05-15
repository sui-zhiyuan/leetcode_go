pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
    let mut minium = grid.clone();
    let mut result = grid[0][1] - grid[0][0];
    for (i , row) in grid.iter().enumerate(){
        for (j , cell) in row.iter().enumerate(){
            let mut min = i32::MAX;
            if i > 0{
                min = min.min(minium[i-1][j]);
                min = min.min(grid[i-1][j]);
            }
            if j> 0 {
                min = min.min(minium[i][j-1]);
                min = min.min(grid[i][j-1]);
            }
            minium[i][j] = min;
            if min < i32::MAX{
                result = result.max(cell - min);
            }
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use crate::common::{self, parse_grid};

    use super::*;

    #[test]
    fn test_1() {
        let grid = parse_grid::<i32>("[[9,5,7,3],[8,9,6,1],[6,7,14,3],[2,5,3,1]]").unwrap();
        assert_eq!(max_score(grid), 9);
    }

    #[test]
    fn test_2() {
        let grid = parse_grid::<i32>("[[4,3,2],[3,2,1]]").unwrap();
        assert_eq!(max_score(grid), -1);
    }
}