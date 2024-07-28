use std::ops::{Add, Sub};

pub fn number_of_substrings(s: String) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let n = s.len();
    let mut states = vec![
        State {
            count_1: 0,
            count_0: 0
        };
        s.len() + 1
    ];

    for (i, &c) in s.iter().enumerate() {
        if c == '1' {
            states[i + 1] = states[i]
                + State {
                    count_1: 1,
                    count_0: 0,
                };
        } else {
            states[i + 1] = states[i]
                + State {
                    count_1: 0,
                    count_0: 1,
                };
        }
    }

    let mut result = 0;
    for i in 0..=n {
        let mut j = i+1;
        while j <=n  {
            // dbg!((i, j, states[j] - states[i], (states[j] - states[i]).check()));
            let check = (states[j] - states[i]).check();
            if check >=0 {
                result += 1;
                j +=1;
                continue;
            }
            let required_1 = states[j].count_1 + (-check) as usize;
            j = binary_search(j+1, n+1, |k| {
                if k > n { return true;}
                states[k].count_1 >= required_1
            });
        }
    }

    result
}

fn binary_search(l:usize, r:usize , check: impl Fn(usize) -> bool) -> usize {
    let mut l = l;
    let mut r = r;
    while r - l > 1 {
        let m = (l + r) / 2;
        if check(m) {
            r = m;
        } else {
            l = m;
        }
    }

    if check(l) {
        l
    } else {
        r
    }
}

#[derive(Clone, Copy, Debug)]
struct State {
    count_1: usize,
    count_0: usize,
}

impl Sub for State {
    type Output = State;

    fn sub(self, rhs: Self) -> Self::Output {
        State {
            count_1: self.count_1 - rhs.count_1,
            count_0: self.count_0 - rhs.count_0,
        }
    }
}

impl Add for State {
    type Output = State;

    fn add(self, rhs: Self) -> Self::Output {
        State {
            count_1: self.count_1 + rhs.count_1,
            count_0: self.count_0 + rhs.count_0,
        }
    }
}

impl State {
    fn check(&self) -> isize {
        self.count_1 as isize - (self.count_0 * self.count_0) as isize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(number_of_substrings("00011".to_string()), 5);
    }
}
