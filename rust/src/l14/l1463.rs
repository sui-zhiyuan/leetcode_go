use crate::common::ExtendVec;

pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid[0].len() as isize;

    let mut curr = &mut ExtendVec::new(-1, n + 1, ExtendVec::new(-1, n + 1, Option::<i32>::None));
    curr[0][n-1] = Some(grid[0][0] + grid[0][n as usize - 1]);
    let mut next = &mut ExtendVec::new(-1, n + 1, ExtendVec::new(-1, n + 1, Option::<i32>::None));

    for row in grid.iter().skip(1) {
        for left in 0..n {
            for right in left..n {
                let mut vn = None;
                for i in 0..9 {
                    let vl = left + (i / 3) - 1;
                    let vr = right + (i % 3) -1 ;
                    vn = vn.max(curr[vl][vr]);
                }

                let add = match left == right {
                    true => row[left as usize],
                    false => row[left as usize] + row[right as usize],
                };

                next[left][right] = vn.map(|vn| vn + add);
            }
        }

        std::mem::swap(&mut curr, &mut next);
        next.iter_mut().for_each(|v| v.fill(None));
    }
    curr.iter().flatten().max().unwrap().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::common;

    use super::*;

    #[test]
    fn test1() {
        let grid = common::parse_grid::<i32>("[[3,1,1],[2,5,1],[1,5,5],[2,1,1]]").unwrap();
        assert_eq!(cherry_pickup(grid), 24);
    }

    #[test]
    fn test2() {
        let grid = common::parse_grid::<i32>(
            "[[1,0,0,0,0,0,1],[2,0,0,0,0,3,0],[2,0,9,0,0,0,0],[0,3,0,5,4,0,0],[1,0,2,3,0,0,6]]",
        )
        .unwrap();
        assert_eq!(cherry_pickup(grid), 28);
    }

    #[test]
    fn test3() {
        let grid = common::parse_grid::<i32>("[[1,0,0,3],[0,0,0,3],[0,0,3,3],[9,0,3,3]]").unwrap();
        assert_eq!(cherry_pickup(grid), 22);
    }

    #[test]
    fn test4() {
        let grid = common::parse_grid::<i32>("[[1,1],[1,1]]").unwrap();
        assert_eq!(cherry_pickup(grid), 4);
    }
}
