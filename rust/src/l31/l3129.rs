use std::cmp;

pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
    let zero = zero as usize;
    let one = one as usize;
    let limit = limit as usize;
    let mut grid = Grid::new(zero, one);
    *grid.get(0, 0, 0) = (1, 1);
    *grid.get(0, 0, 1) = (1, 1);
    for s0 in 1..=cmp::min(zero, limit) {
        *grid.get(s0, 0, 0) = (1, 1);
        *grid.get(s0, 0, 1) = (0, 1)
    }
    for s1 in 1..=cmp::min(one, limit) {
        *grid.get(0, s1, 0) = (0, 1);
        *grid.get(0, s1, 1) = (1, 1)
    }

    for s0 in 1..=zero {
        for s1 in 1..=one {
            let count = grid.get(s0 - 1, s1, 1).1;
            let mut sum = grid.get(s0, s1 - 1, 0).1 as i64;
            sum += count as i64;
            if s1 >= limit {
                sum -= grid.get(s0, s1 - limit, 0).0 as i64;
            }
            sum += MODE as i64;
            sum %= MODE as i64;
            // dbg!((s0, s1, 0, count, sum));
            *grid.get(s0, s1, 0) = (count, sum as i32);

            let count = grid.get(s0, s1 - 1, 0).1;
            let mut sum = grid.get(s0 - 1, s1, 1).1 as i64;
            sum += count as i64;
            if s0 >= limit {
                sum -= grid.get(s0 - limit, s1, 1).0 as i64;
            }
            sum += MODE as i64;
            sum %= MODE as i64;
            // dbg!((s0, s1, 1, count, sum));
            *grid.get(s0, s1, 1) = (count, sum as i32);
        }
    }

    let v1 = grid.get(zero, one, 0).0;
    let v2 = grid.get(zero, one, 1).0;

    (v1 + v2) % MODE
}

const MODE: i32 = 1000000007;

struct Grid {
    value: Vec<(i32, i32)>,
    zero: usize,
    one: usize,
}

impl Grid {
    fn new(zero: usize, one: usize) -> Self {
        Self {
            value: vec![(0, 0); (zero + 1) * (one + 1) * 2],
            zero: zero + 1,
            one: one + 1,
        }
    }

    fn get(&mut self, zero: usize, one: usize, b: usize) -> &mut (i32, i32) {
        &mut self.value[zero * self.one * 2 + one * 2 + b]
    }
}
