use std::{
    cell::RefCell, collections::{HashMap, VecDeque}, ops::Add
};

pub fn max_moves(kx: i32, ky: i32, positions: Vec<Vec<i32>>) -> i32 {
    let mut pos = vec![Square::new(kx, ky)];
    pos.extend(positions.iter().map(|p| Square::new(p[0], p[1])));

    let mut distances = vec![vec![0; pos.len()]; pos.len()];
    for i in 0..pos.len() {
        for j in i + 1..pos.len() {
            let d = pos[i].distance(&pos[j]);
            distances[i][j] = d;
            distances[j][i] = d;
        }
    }

    let ctx = Context {
        distances,
        cache: RefCell::new(HashMap::new()),
        all_visited: (1 << pos.len()) - 1,
    };

    dfs(&ctx, 1, 0, State::FindMax)
}

struct Context {
    distances: Vec<Vec<i32>>,
    cache: RefCell<HashMap<(usize, u64), i32>>,
    all_visited: u64,
}

fn dfs(
    ctx: &Context,
    visited: u64,
    curr: usize,
    state: State,
) -> i32 {
    if visited == ctx.all_visited {
        return 0;
    }
    if let Some(&result) = ctx.cache.borrow().get(&(curr, visited)) {
        return result;
    }

    let mut result = match state {
        State::FindMax => 0,
        State::FindMin => i32::MAX,
        
    };
    for i in 0..ctx.distances.len() {
        if visited & (1 << i) != 0 {
            continue;
        }

        let new_visited = visited | (1 << i);
        let value = ctx.distances[curr][i] + dfs(ctx, new_visited, i, state.other());
        result = match state {
            State::FindMax => result.max(value),
            State::FindMin => result.min(value),
        };
    }

    ctx.cache.borrow_mut().insert((curr, visited), result);
    result
}

enum State {
    FindMax,
    FindMin,
}

impl State{
    fn other(&self) -> Self {
        match self {
            Self::FindMax => Self::FindMin,
            Self::FindMin => Self::FindMax,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Square {
    row: i32,
    col: i32,
}

impl Square {
    fn new(row: i32, col: i32) -> Self {
        Square { row, col }
    }

    fn verify(&self) -> bool {
        self.row >= 0 && self.row <= 49 && self.col >= 0 && self.col <= 49
    }

    fn all_moves(&self) -> Vec<Square> {
        let moves = [
            Self::new(-2, -1),
            Self::new(-2, 1),
            Self::new(-1, -2),
            Self::new(-1, 2),
            Self::new(1, -2),
            Self::new(1, 2),
            Self::new(2, -1),
            Self::new(2, 1),
        ];
        let mut result = Vec::new();
        for m in moves {
            let new = *self + m;
            if !new.verify() {
                continue;
            }
            result.push(new);
        }
        result
    }

    fn distance(&self, target: &Square) -> i32 {
        let mut queue = VecDeque::new();
        let mut visited = HashMap::<Square, i32>::new();

        visited.insert(*self, 0);
        queue.push_back(*self);
        while let Some(curr) = queue.pop_front() {
            if curr == *target {
                return visited[&curr];
            }

            for next in curr.all_moves() {
                if *visited.get(&next).unwrap_or(&i32::MAX) > visited[&curr] + 1 {
                    visited.insert(next, visited[&curr] + 1);
                    queue.push_back(next);
                }
            }
        }
        unreachable!()
    }
}

impl Add for Square {
    type Output = Square;

    fn add(self, other: Square) -> Square {
        Square::new(self.row + other.row, self.col + other.col)
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_max_moves(){
        let kx = 1;
        let ky = 1;
        let positions = [[0,0]];
        let positions = positions.iter().map(|p| p.to_vec()).collect();
        assert_eq!(max_moves(kx, ky, positions), 4);
    }


    #[test]
    fn test2(){
        let kx = 0;
        let ky = 0;
        let positions = [[1,1],[2,2],[3,3],[4,4],[5,5],[6,6],[7,7],[8,8],[9,9],[10,10],[11,11],[12,12],[13,13],[14,14],[15,15]];
        let positions = positions.iter().map(|p| p.to_vec()).collect();
        assert_eq!(max_moves(kx, ky, positions), 70);
    }
}