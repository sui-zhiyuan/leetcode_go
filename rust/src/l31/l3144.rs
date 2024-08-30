use std::{iter, ops::Sub};

pub fn minimum_substrings_in_partition(s: String) -> i32 {
    let s = to_int(&s);

    let count = s.iter().scan(Count::default(), |state, &v| {
        state.count[v] += 1;
        Some(*state)
    });
    let count = iter::once(Count::default())
        .chain(count)
        .collect::<Vec<_>>();

    let mut state = vec![i32::MAX; count.len()];
    state[0] = 0;
    for i in 1..count.len() {
        for j in 0..i {
            if (count[i] - count[j]).check() {
                state[i] = state[i].min(state[j] + 1);
            }
        }
    }

    state.last().copied().unwrap_or(0) as i32
}

fn to_int(s: &str) -> Vec<usize> {
    s.chars()
        .inspect(|v| {
            if !v.is_ascii_lowercase() {
                panic!("invalid value")
            }
        })
        .map(|c| c as usize - 'a' as usize)
        .collect()
}

#[derive(Debug, Clone, Copy, Default)]
struct Count {
    count: [usize; 26],
}

impl Count {
    fn check(&self) -> bool {
        let Some(&first_count) = self.count.iter().find(|&&v| v > 0) else {
            return true;
        };
        self.count.iter().all(|&v| v == 0 || v == first_count)
    }
}

impl Sub for Count {
    type Output = Count;

    fn sub(self, rhs: Self) -> Self::Output {
        let count = self
            .count
            .iter()
            .zip(rhs.count.iter())
            .map(|(&a, &b)| a - b)
            .collect::<Vec<_>>();

        Count {
            count: count.try_into().unwrap(),
        }
    }
}
