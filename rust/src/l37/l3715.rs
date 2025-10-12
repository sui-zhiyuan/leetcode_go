use crate::common::prime_list;
use std::collections::HashMap;

pub fn sum_of_ancestors(n: i32, edges: Vec<Vec<i32>>, nums: Vec<i32>) -> i64 {
    const MAX_VALUE: i32 = 100009;
    let primes = prime_list(MAX_VALUE);

    assert_eq!(n as usize, nums.len());

    let mut graph = nums
        .iter()
        .map(|&v| Node {
            edges: vec![],
            value: square_remain(&primes, v),
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

fn square_remain(primes: &[i32], v: i32) -> i32 {
    let mut v = v;
    for &p in primes {
        let Some(ps) = p.checked_mul(p) else {
            continue;
        };
        while v % ps == 0 {
            v /= ps;
        }
    }
    v
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::common::{parse_grid, prime_list};
    use crate::parameter_tests;

    #[test]
    fn prime_count() {
        let n = prime_list(100009);

        assert!(1200 * 8 > n.len());
        println!("{}", n.len())
    }

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
