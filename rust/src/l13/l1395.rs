use std::{cmp::Ordering, fmt::Debug};

pub fn num_teams(rating: Vec<i32>) -> i32 {
    let mut states = vec![State{up: [0; 3], down: [0; 3]}; rating.len()];

    let mut result = 0;
    for (i , v) in rating.iter().enumerate() {
        let mut curr = State{up: [0; 3], down: [0; 3]};
        curr.up[0] = 1;
        curr.down[0] = 1;
        for j in 0..i {
            match v.cmp(&rating[j]) {
                Ordering::Greater => {
                    curr.up[1] += states[j].up[0];
                    curr.up[2] += states[j].up[1];
                }
                Ordering::Less => {
                    curr.down[1] += states[j].down[0];
                    curr.down[2] += states[j].down[1];
                }
                Ordering::Equal => {}
            }
        }

        result += curr.up[2] + curr.down[2];

        states[i] = curr;
        dbg!(&states);
    }

    result
}

#[derive(Clone)]
struct State{
    up: [i32 ;3],
    down: [i32 ;3],
}

// debug and test

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let up = format!("[{}, {}, {}]", self.up[0], self.up[1], self.up[2]);
        let down = format!("[{}, {}, {}]", self.down[0], self.down[1], self.down[2]);
        f.debug_struct("State")
            .field("up", &up)
            .field("down", &down)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let rating = vec![2,5,3,4,1];
        let res = 3;
        assert_eq!(num_teams(rating), res);
    }

    #[test]
    fn test2() {
        let rating = vec![1,2,3,4];
        let res = 4;
        assert_eq!(num_teams(rating), res);
    }
}