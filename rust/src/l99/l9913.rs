use std::cmp;

pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
    let zero = zero as usize;
    let one = one as usize;
    let limit = limit as usize;
    let mut grid = Grid::new(zero, one);

    for s0 in 0..=cmp::min(zero, limit) {
        *grid.get(s0, 0, 0) = 1;
    }
    for s1 in 0..=cmp::min(one, limit) {
        *grid.get(0, s1, 1) = 1;
    }

    for s0 in 1..=zero {
        for s1 in 1..=one {
            let mut count = 0;
            for k in 1..=limit {
                if s0 >= k {
                    count += *grid.get(s0 - k, s1, 1);
                    //dbg!((s0 - k, s1, 1 , *grid.get(s0 - k, s1, 1)));
                    count %= MODE;
                }
            }
            //dbg!((s0, s1, 0 , count));
            *grid.get(s0, s1, 0) = count;

            let mut count = 0;
            for k in 1..=limit {
                if s1 >= k {
                    count += *grid.get(s0, s1 - k, 0);
                    // dbg!((s0, s1 - k, 0 , *grid.get(s0, s1 - k, 0)));
                    count %= MODE;
                }
            }
            //dbg!((s0, s1, 1 , count));
            *grid.get(s0, s1, 1) = count;
        }
    }

    let v1 = *grid.get(zero, one, 0);
    let v2 = *grid.get(zero, one, 1);

    (v1 + v2) % MODE
}

const MODE: i32 = 1000000007;

struct Grid {
    value: Vec<i32>,
    zero: usize,
    one: usize,
}

impl Grid {
    fn new(zero: usize, one: usize) -> Self {
        Self {
            value: vec![0; (zero + 1) * (one + 1) * 2],
            zero:zero+1,
            one:one+1,
        }
    }

    fn get(&mut self, zero: usize, one: usize, b: usize) -> &mut i32 {
        &mut self.value[zero * self.one * 2 + one * 2 + b]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(2, number_of_stable_arrays(1, 1, 2));
    }

    #[test]
    fn test2() {
        assert_eq!(1, number_of_stable_arrays(1, 2, 1));
    }

    #[test]
    fn test3() {
        assert_eq!(14, number_of_stable_arrays(3, 3, 2));
    }

    #[test]
    fn test4() {
        assert_eq!(6, number_of_stable_arrays(2, 2, 100));
    }
}
