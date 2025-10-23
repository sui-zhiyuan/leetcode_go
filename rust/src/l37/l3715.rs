use std::collections::HashMap;

pub fn sum_of_ancestors(_n: i32, edges: Vec<Vec<i32>>, nums: Vec<i32>) -> i64 {
    const MAX_VALUE: i32 = 100009;
    let square_remain = get_square_remain(MAX_VALUE as usize);

    let mut graph = nums
        .iter()
        .map(|&v| Node {
            edges: vec![],
            value: square_remain[v as usize] as i32,
        })
        .collect::<Vec<_>>();

    for e in edges {
        let &[left, right] = &e[..] else {
            panic!("invalid input edge")
        };

        let left = left as usize;
        let right = right as usize;

        graph[left].edges.push(right);
        graph[right].edges.push(left);
    }

    let ctx = Ctx { graph };

    let mut visited = HashMap::new();

    dfs(&ctx, &mut visited, 0, 0)
}

struct Ctx {
    graph: Vec<Node>,
}

#[derive(Clone)]
struct Node {
    edges: Vec<usize>,
    value: i32,
}

fn dfs(ctx: &Ctx, visited: &mut HashMap<i32, i64>, curr: usize, parent: usize) -> i64 {
    let value = ctx.graph[curr].value;
    let &(mut count) = visited.get(&value).unwrap_or(&0);

    *visited.entry(value).or_default() += 1;
    for &next in ctx.graph[curr].edges.iter() {
        if next == parent {
            continue;
        }

        let sub_count = dfs(ctx, visited, next, curr);
        count += sub_count;
    }

    *visited.entry(value).or_default() -= 1;
    count
}

fn get_square_remain(max: usize) -> Vec<usize> {
    let mut square_remain = vec![0; max + 1];

    for i in 1..=max {
        if square_remain[i] != 0 {
            continue;
        }

        for j in 1..=max {
            let pow = i.saturating_mul(j).saturating_mul(j);
            if pow >= square_remain.len() {
                break;
            }
            square_remain[pow] = i;
        }
    }

    square_remain
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::common::parse_grid;
    use crate::parameter_tests;

    struct Case {
        n: i32,
        edges: &'static str,
        nums: Vec<i32>,
        expect: i64,
    }

    fn test(c: Case) {
        let edges = parse_grid::<i32>(c.edges).unwrap();

        assert_eq!(c.expect, sum_of_ancestors(c.n, edges, c.nums))
    }

    parameter_tests! {
        test,
        (case1: Case {
            n: 3,
            edges: "[[0,1],[1,2]]",
            nums: vec![2,8,2],
            expect: 3,
        }),
        (case2: Case {
            n: 3,
            edges: "[[0,1],[0,2]]",
            nums: vec![1,2,4],
            expect: 1,
        }),
        (case3: Case {
            n: 4,
            edges: "[[0,1],[0,2],[1,3]]",
            nums: vec![1,2,9,4],
            expect: 2,
        }),
        (case4: Case {
            n: 3,
            edges: "[[0,1],[0,2]]",
            nums: vec![40622,40622,40622],
            expect: 2,
        }),
    }
}
