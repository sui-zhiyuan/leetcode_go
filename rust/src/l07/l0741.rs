pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let curr = &mut vec![vec![Some(0i32); n]; n];
    let next = &mut vec![vec![Option::<i32>::None; n]; n];

    for k in 0..2 * n - 1 {
        for x1 in k.max(n - 1) + 1 - n..(k + 1).min(n) {
            for x2 in x1..(k + 1).min(n) {
                let v1 = match grid[x1][k - x1] {
                    -1 => None,
                    v => Some(v),
                };
                let v2 = match grid[x2][k - x2] {
                    -1 => None,
                    v => Some(v),
                };
                let vc = match x1 == x2 {
                    true => v1,
                    false => v1.and_then(|v1| v2.map(|v2| v1 + v2)),
                };

                let mut v = curr[x1][x2];
                if x1 > 0 {
                    v = v.max(curr[x1 - 1][x2]);
                }
                if x2 > 0 {
                    v = v.max(curr[x1][x2 - 1]);
                }
                if x1 > 0 && x2 > 0 {
                    v = v.max(curr[x1 - 1][x2 - 1]);
                }
                next[x1][x2] = v.and_then(|v| vc.map(|vc| v + vc));
                // print!("({} {}):{:?} ", x1, x2, next[x1][x2]);
            }
        }
        // println!();
        std::mem::swap(curr, next);
        next.iter_mut().for_each(|v| v.fill(None));
    }

    curr[n - 1][n - 1].unwrap_or(0) as i32
}

#[cfg(test)]
mod tests {
    use crate::common;

    use super::*;

    #[test]
    fn test1() {
        let grid = common::parse_grid::<i32>("[[0,1,-1],[1,0,-1],[1,1,1]]").unwrap();
        assert_eq!(cherry_pickup(grid), 5);
    }

    #[test]
    fn test2() {
        let grid = common::parse_grid::<i32>("[[1,1,-1],[1,-1,1],[-1,1,1]]").unwrap();
        assert_eq!(cherry_pickup(grid), 0);
    }
}
