use std::collections::HashMap;

pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut grid_sum = vec![HashMap::<i32, usize>::new(); m];
    for row in grid.iter() {
        for (j, &cell) in row.iter().enumerate() {
            *grid_sum[j].entry(cell).or_insert(n) -= 1
        }
    }

    let mut value = -1;
    // for any else number
    for col in grid_sum.iter_mut() {
        col.insert(value, n);
        value = -3 - value;
    }

    let mut curr = &mut HashMap::<i32, usize>::new();
    let mut next = &mut HashMap::<i32, usize>::new();
    curr.insert(-2, 0);
    for col in grid_sum.iter() {
        for (&v, &count) in col.iter() {
            let min = curr
                .iter()
                .filter(|&(&pv, _)| pv != v)
                .map(|(_, pc)| pc)
                .min()
                .unwrap();
            next.insert(v, min + count);
        }
        (curr, next) = (next, curr);
        next.clear();
    }
    curr.iter().map(|(_, &v)| v).min().unwrap() as i32
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
