use std::mem;

pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut curr = Vec::<(usize, usize)>::new();
    let mut next = Vec::<(usize, usize)>::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 2 {
                curr.push((i, j));
            }
        }
    }

    let mut round = 0;
    while !curr.is_empty() {
        while let Some((x, y)) = curr.pop() {
            if x > 0 && grid[x - 1][y] == 1 {
                grid[x - 1][y] = 2;
                next.push((x - 1, y));
            }
            if x < grid.len() - 1 && grid[x + 1][y] == 1 {
                grid[x + 1][y] = 2;
                next.push((x + 1, y));
            }
            if y > 0 && grid[x][y - 1] == 1 {
                grid[x][y - 1] = 2;
                next.push((x, y - 1));
            }
            if y < grid[0].len() - 1 && grid[x][y + 1] == 1 {
                grid[x][y + 1] = 2;
                next.push((x, y + 1));
            }
        }

        if !next.is_empty() {
            round += 1;
        }
        mem::swap(&mut curr, &mut next);
    }

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 1 {
                return -1;
            }
        }
    }

    round
}

#[cfg(test)]
mod test {
    use crate::common;

    use super::*;

    #[test]
    fn test1() {
        let grid = common::parse_grid::<i32>("[[2,1,1],[1,1,0],[0,1,1]]").unwrap();
        assert_eq!(oranges_rotting(grid), 4)
    }
}
