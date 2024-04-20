use std::{cmp, collections::BTreeSet};

pub fn maximum_minimum_path(grid: Vec<Vec<i32>>) -> i32 {
    let max_score = cmp::min(grid[0][0], grid[grid.len() - 1][grid[0].len() - 1]);
    let values = BTreeSet::from_iter(
        grid.iter()
            .flat_map(|v| v.iter().copied())
            .filter(|&v| v <= max_score),
    );

    let mut reached = vec![vec![false; grid[0].len()]; grid.len()];
    reached[0][0] = true;
    let mut curr = &mut vec![(0, 0)];
    let mut next = &mut Vec::<(usize, usize)>::new();
    for &cv in values.iter().rev() {
        while let Some((i, j)) = curr.pop() {
            if grid[i][j] < cv {
                next.push((i, j));
                continue;
            }
            for (ni, nj) in get_next(i, j, grid.len(), grid[0].len()) {
                if !reached[ni][nj] && grid[ni][nj] >= cv {
                    reached[ni][nj] = true;
                    curr.push((ni, nj));
                    continue;
                }
                if !reached[ni][nj] {
                    next.push((ni, nj));
                }
            }
        }

        // println!("{} ->", cv);
        // for row in reached.iter() {
        //     for &cell in row.iter() {
        //         print!("{}", if cell { "X" } else { "O" });
        //     }
        //     println!();
        // }
        // println!();

        if reached[grid.len() - 1][grid[0].len() - 1] {
            return cv;
        }
        (curr, next) = (next, curr);
    }

    unreachable!();
}

fn get_next(i: usize, j: usize, m: usize, n: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    if i > 0 {
        result.push((i - 1, j));
    }
    if i < m - 1 {
        result.push((i + 1, j));
    }
    if j > 0 {
        result.push((i, j - 1));
    }
    if j < n - 1 {
        result.push((i, j + 1));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let grid = vec![vec![5, 4, 5], vec![1, 2, 6], vec![7, 4, 6]];
        let result = maximum_minimum_path(grid);
        assert_eq!(result, 4);
    }

    #[test]
    fn test2() {
        let grid = vec![
            vec![5, 0, 3, 5, 4, 1],
            vec![3, 5, 1, 1, 2, 5],
            vec![3, 5, 5, 5, 4, 0],
            vec![2, 0, 3, 0, 5, 5],
            vec![1, 4, 5, 0, 0, 5],
        ];
        let result = maximum_minimum_path(grid);
        assert_eq!(result, 3);
    }
}
