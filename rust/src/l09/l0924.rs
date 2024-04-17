use std::ops::Add;

use crate::common::DisjointSet;

impl Solution {
    pub fn min_malware_spread(graph: Vec<Vec<i32>>, initial: Vec<i32>) -> i32 {
        let n = graph.len();
        let mut effected = vec![0; n];
        for i in initial.iter() {
            effected[(*i) as usize] = 1;
        }

        let mut dsu = DisjointSet::new(&effected, <i32 as Add>::add);
        for (i, row) in graph.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate().take(i) {
                if cell == 1 {
                    dsu.union(i, j);
                }
            }
        }

        let mut initial = initial;
        initial.sort_unstable();
        let mut effect = 0;
        let mut result = initial[0];
        for i in initial {
            let size = if dsu.value(i.try_into().unwrap()) >= 2 {
                0
            } else {
                dsu.size(i.try_into().unwrap())
            };
            if size > effect {
                effect = size;
                result = i;
            }
        }

        result
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let graph = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
        let initial = vec![0, 1];
        assert_eq!(Solution::min_malware_spread(graph, initial), 0);
    }

    #[test]
    fn test2() {
        let graph = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        let initial = vec![0, 2];
        assert_eq!(Solution::min_malware_spread(graph, initial), 0);
    }

    #[test]
    fn test3() {
        let graph = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
        let initial = vec![1, 2];
        assert_eq!(Solution::min_malware_spread(graph, initial), 1);
    }
}
