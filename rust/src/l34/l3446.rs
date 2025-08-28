use crate::common::Grid;
use std::cmp::Reverse;

pub fn sort_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut grid = Grid::from(grid);

    for i in 0..grid.size().x {
        let max_j = grid.size().y.min(grid.size().x - i);

        let mut values = Vec::with_capacity(max_j);
        for j in 0..max_j {
            let x = i + j;
            let y = j;
            values.push(grid[(x, y)]);
        }

        values.sort_unstable_by_key(|&v| Reverse(v));

        for j in 0..max_j {
            let x = i + j;
            let y = j;
            grid[(x, y)] = values[j];
            values.push(grid[(x, y)]);
        }
    }

    for j in 1..grid.size().y {
        let max_i = grid.size().x.min(grid.size().x - j);

        let mut values = Vec::with_capacity(max_i);
        for i in 0..max_i {
            let x = i;
            let y = j + i;
            values.push(grid[(x, y)]);
        }

        values.sort_unstable_by_key(|&v| v);

        for i in 0..max_i {
            let x = i;
            let y = j + i;
            grid[(x, y)] = values[i];
        }
    }

    grid.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::parse_grid;

    #[test]
    fn test1() {
        let input = parse_grid::<i32>("[[1,7,3],[9,8,2],[4,5,6]]").unwrap();
        let expect = parse_grid::<i32>("[[8,2,3],[9,6,7],[4,5,1]]").unwrap();

        assert_eq!(expect, sort_matrix(input))
    }
}
