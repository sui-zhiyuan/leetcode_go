use std::{mem, ops::{Index, IndexMut}};

pub fn check_record(n: i32) -> i32 {
    let mut curr = vec![0i64; 6];
    let mut next = vec![0i64; 6];

    const MOD: i64 = 1_000_000_007;
    curr[Pos { absent: 0, late: 0 }] = 1;
    let all_pos = Pos::all().collect::<Vec<_>>();
    for _ in 0..n {
        for p in all_pos.iter(){
            if p.absent < 1 {
                next[Pos { absent: p.absent + 1, late: 0 }] += curr[*p];
            }
            if p.late < 2 {
                next[Pos { absent: p.absent, late: p.late + 1 }] += curr[*p];
            }
            next[Pos { absent: p.absent, late: 0 }] += curr[*p];
        }

        for v in next.iter_mut(){
            *v %= MOD;
        }

        mem::swap(&mut curr, &mut next);
        next.fill(0);
    }

    (curr.iter().sum::<i64>() % MOD) as i32
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    absent: usize,
    late: usize,
}

impl Pos {
    fn all() -> impl Iterator<Item = Pos> {
        (0..2).flat_map(move |absent| (0..3).map(move |late| Pos { absent, late }))
    }
}

impl Index<Pos> for Vec<i64> {
    type Output = i64;

    fn index(&self, pos: Pos) -> &Self::Output {
        &self[pos.absent * 3 + pos.late]
    }
}

impl IndexMut<Pos> for Vec<i64> {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        &mut self[pos.absent * 3 + pos.late]
    }
}
