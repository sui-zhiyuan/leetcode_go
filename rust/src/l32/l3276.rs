use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
    let mut count = HashMap::<i32, i32>::new();
    let mut grid = grid;

    for row in grid.iter_mut() {
        row.sort_unstable_by_key(|v| -v);

        for v in row.iter() {
            *count.entry(*v).or_insert(0) += 1;
        }
    }

    let total_sum = grid.iter().map(|row| row[0]).sum::<i32>();
    let mut original_edge = 0;
    let mut taken = HashSet::<i32>::new();
    for row in grid.iter() {
        let mut j = 0;
        while j < row.len() && taken.contains(&row[j]) {
            j += 1;
        }
        if j == row.len() {
            original_edge += row[0];
        } else {
            original_edge += row[0] - row[j];
            taken.insert(row[j]);
        }
    }

    let ctx = Ctx {
        grid,
        edge: RefCell::new(original_edge),
    };

    let mut taken = HashSet::<i32>::new();
    dfs(&ctx, &mut count, &mut taken, 0, 0);

    let edge = *ctx.edge.borrow();
    total_sum - edge
}

struct Ctx {
    grid: Vec<Vec<i32>>,
    edge: RefCell<i32>,
}

fn dfs(ctx: &Ctx, count: &mut HashMap<i32, i32>, taken: &mut HashSet<i32>, line: usize, used: i32) {
    if used >= *ctx.edge.borrow() {
        return;
    }
    if line == ctx.grid.len() {
        *ctx.edge.borrow_mut() = used;
        return;
    }

    let mut removed = Vec::new();
    for v in ctx.grid[line].iter() {
        if taken.contains(v) {
            continue;
        }

        taken.insert(*v);
        dfs(ctx, count, taken, line + 1, used + ctx.grid[line][0] - v);
        taken.remove(v);
        *count.entry(*v).or_insert(0) -= 1;
        removed.push(*v);
        if count[v] <= 0 {
            break;
        }
    }

    dfs(ctx, count, taken, line + 1, used + ctx.grid[line][0]);
    for v in removed {
        *count.entry(v).or_insert(0) += 1;
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let grid = [[18,12,4,13],[4,7,13,13],[16,20,15,12],[10,3,18,9]];
        let grid = grid.iter().map(|row| row.to_vec()).collect();
        assert_eq!(max_score(grid), 63);
    }
}