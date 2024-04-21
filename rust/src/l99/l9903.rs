use std::{cmp, collections::HashMap};

pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut grid_sum = vec![HashMap::<i32, usize>::new(); m];
    for row in grid.iter() {
        for (j, &cell) in row.iter().enumerate() {
            *grid_sum[j].entry(cell).or_default() += 1
        }
    }
    for (i, col) in grid_sum.iter_mut().enumerate() {
        for (_, v) in col.iter_mut() {
            *v = n - *v;
        }
        // for any else number
        col.insert(-(i as i32) - 100, n);
    }

    // dbg!(&grid_sum);

    let mut curr = &mut HashMap::<i32, usize>::new();
    let mut next = &mut HashMap::<i32, usize>::new();
    curr.insert(-1, 0);
    for col in grid_sum.iter() {
        for (&v, &count) in col.iter() {
            let mut min = usize::MAX;
            for (&pv, &pc) in curr.iter() {
                if pv == v {
                    continue;
                }
                min = cmp::min(pc, min);
            }
            if min == usize::MAX {
                unreachable!("no value found")
            }
            next.insert(v, min + count);
        }
        (curr, next) = (next, curr);
        next.clear();
        // dbg!(&curr);
    }
    let mut min = usize::MAX;
    for (_, &v) in curr.iter() {
        min = cmp::min(min, v);
    }
    min.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_operations() {
        assert_eq!(
            minimum_operations(vec![vec![1, 2, 3], vec![3, 4, 5], vec![3, 4, 5]]),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(minimum_operations(vec![vec![1, 1, 1], vec![0, 0, 0]]), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(minimum_operations(vec![vec![1], vec![2], vec![3]]), 2);
    }

    #[test]
    fn test4() {
        assert_eq!(
            minimum_operations(vec![vec![2, 6, 6, 9, 8, 4, 2, 6, 2, 3]]),
            1
        );
    }
}
