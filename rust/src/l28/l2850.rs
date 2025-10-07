pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
    let mut more = Vec::new();
    let mut less = Vec::new();

    for (r, row) in grid.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            if v == 1 {
                continue;
            }

            if v == 0 {
                less.push(Point { row: r, col: c });
            }

            for _i in 2..=v {
                more.push(Point { row: r, col: c });
            }
        }
    }

    assert!(more.len() == less.len());

    let mut less_used = vec![false; less.len()];

    all_match(&more, &less, &mut less_used, 0)
}

struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn distance(&self, other: &Point) -> i32 {
        let v = self.row.abs_diff(other.row) + self.col.abs_diff(other.col);
        v as i32
    }
}

fn all_match(more: &[Point], less: &[Point], less_used: &mut [bool], curr: usize) -> i32 {
    if curr == more.len() {
        return 0;
    }

    let mut result = i32::MAX;

    for i in 0..less.len() {
        if less_used[i] {
            continue;
        }

        less_used[i] = true;
        let dis = more[curr].distance(&less[i]);
        let curr = dis + all_match(more, less, less_used, curr + 1);
        result = result.min(curr);
        less_used[i] = false;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let grid = [[1, 1, 0], [1, 1, 1], [1, 2, 1]];
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert_eq!(minimum_moves(grid), 3);
    }

    #[test]
    fn test2() {
        let grid = [[1, 3, 0], [1, 0, 0], [1, 0, 3]];
        let grid = grid.iter().map(|r| r.to_vec()).collect();
        assert_eq!(minimum_moves(grid), 4);
    }
}
