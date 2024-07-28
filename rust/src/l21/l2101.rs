pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
    let bombs = bombs.iter().map(|v| Bomb::from_vec(v)).collect::<Vec<_>>();
    let n = bombs.len();

    let mut grid = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            if bombs[i].in_range(&bombs[j]) {
                grid[i][j] = 1;
            }
        }
    }

    for (i , row) in grid.iter_mut().enumerate() {
        row[i] = 1;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if grid[i][k] == 1 && grid[k][j] == 1 {
                    grid[i][j] = 1;
                }
            }
        }
    }

    grid.iter().map(|v| v.iter().sum::<i32>()).max().unwrap()
}

struct Bomb {
    x: i32,
    y: i32,
    r: i32,
}

impl Bomb {
    fn from_vec(v: &[i32]) -> Bomb {
        Bomb {
            x: v[0],
            y: v[1],
            r: v[2],
        }
    }

    fn in_range(&self, other: &Bomb) -> bool {
        let dx = self.x as i64 - other.x as i64;
        let dy = self.y as i64 - other.y as i64;
        let d = dx * dx + dy * dy;
        d <= self.r as i64 * self.r as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let bombs = [
            [37207, 2653, 5261],
            [40784, 59523, 20635],
            [16390, 1426, 39102],
            [42236, 12, 96855],
            [72839, 62027, 61667],
            [60691, 58191, 48447],
            [42932, 46579, 41248],
            [35868, 43119, 6870],
            [41693, 98905, 17374],
            [43441, 1266, 41621],
        ];
        let bombs = bombs.iter().map(|v| v.to_vec()).collect();
        assert_eq!(maximum_detonation(bombs), 10);
    }
}
