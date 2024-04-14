use std::cell::RefCell;

// cspell:ignore coprimes

use crate::common::Flyweight;

impl Solution {
    pub fn get_coprimes(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let max_num = nums.iter().copied().max().unwrap() as usize;
        let mut nodes = nums
            .into_iter()
            .map(|v| Node {
                visited: RefCell::new(false),
                value: v as usize,
                depth: RefCell::new(0),
                children: Vec::new(),
            })
            .collect::<Vec<_>>();
        edges
            .iter()
            .map(|v| {
                (
                    usize::try_from(v[0]).unwrap(),
                    usize::try_from(v[1]).unwrap(),
                )
            })
            .for_each(|(a, b)| {
                nodes[a].children.push(b);
                nodes[b].children.push(a);
            });

        let mut ans = vec![-1; nodes.len()];
        let ctx = Ctx {
            co_prime: RefCell::new(Flyweight::new(Box::new(move |v| co_prime(v, max_num)))),
            path: RefCell::new(vec![Vec::new(); max_num + 1]),
            nodes,
        };
        *ctx.nodes[0].visited.borrow_mut() = true;
        *ctx.nodes[0].depth.borrow_mut() = 1;

        Self::dfs(&ctx, &mut ans, 0);
        ans
    }

    fn dfs(ctx: &Ctx, ans: &mut [i32], curr: usize) {
        let Ctx {
            co_prime,
            nodes,
            path,
        } = ctx;
        let curr_node = &nodes[curr];

        let mut ans_deep = 0;
        for &co_prom in co_prime.borrow_mut().get(curr_node.value) {
            if let Some(&last) = path.borrow()[co_prom].last() {
                if ans_deep < *nodes[last].depth.borrow() {
                    ans[curr] = last as i32;
                    ans_deep = *nodes[last].depth.borrow();
                }
            }
        }

        path.borrow_mut()[curr_node.value].push(curr);
        for &child in curr_node.children.iter() {
            let child_node = &nodes[child];
            if *child_node.visited.borrow() {
                continue;
            }
            *child_node.depth.borrow_mut() = *curr_node.depth.borrow() + 1;
            *child_node.visited.borrow_mut() = true;

            Self::dfs(ctx, ans, child);
        }
        ctx.path.borrow_mut()[curr_node.value].pop();
    }
}

struct Ctx {
    co_prime: RefCell<Flyweight<usize, Vec<usize>>>,
    path: RefCell<Vec<Vec<usize>>>,
    nodes: Vec<Node>,
}

#[derive(Debug, Clone)]
struct Node {
    visited: RefCell<bool>,
    value: usize,
    depth: RefCell<usize>,
    children: Vec<usize>,
}

fn co_prime(v: usize, max: usize) -> Vec<usize> {
    (1..=max).filter(|j| gcd(v, *j) == 1).collect()
}

fn gcd(a: usize, b: usize) -> usize {
    let (mut a, mut b) = (a, b);
    while b > 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_coprimes_1() {
        let nums = vec![2, 3, 3, 2];
        let edges = vec![vec![0, 1], vec![1, 2], vec![1, 3]];
        let res = Solution::get_coprimes(nums, edges);
        assert_eq!(res, vec![-1, 0, 0, 1]);
    }

    #[test]
    fn test_get_coprimes_2() {
        let nums = vec![5, 6, 10, 2, 3, 6, 15];
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 5],
            vec![2, 6],
        ];
        let res = Solution::get_coprimes(nums, edges);
        assert_eq!(res, vec![-1, 0, -1, 0, 0, 0, -1]);
    }

    #[test]
    fn test_get_coprimes_empty() {
        let nums = vec![1];
        let edges = vec![];
        let res = Solution::get_coprimes(nums, edges);
        assert_eq!(res, vec![-1]);
    }
}
