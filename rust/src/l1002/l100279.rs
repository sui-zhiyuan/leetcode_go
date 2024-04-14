use std::collections::{HashMap, LinkedList};

impl Solution {
    pub fn minimum_time(n: i32, edges: Vec<Vec<i32>>, disappear: Vec<i32>) -> Vec<i32> {
        let mut table = Vec::<HashMap<usize, i32>>::new();
        for _ in 0..n {
            table.push(HashMap::new());
        }

        edges.into_iter().for_each(|v| {
            let (from, to, cost) = (v[0] as usize, v[1] as usize, v[2]);
            if from == to {
                return;
            }
            let v1 = table[from].entry(to).or_insert(cost);
            *v1 = (*v1).min(cost);
            let v2 = table[to].entry(from).or_insert(cost);
            *v2 = (*v2).min(cost);
        });

        let table = table;
        let n = n as usize;

        let mut reach = vec![-1; n];
        let mut queue = LinkedList::<usize>::new();
        let mut visiting = vec![false; n];
        if disappear[0] >= 0 {
            reach[0] = 0;
            queue.push_back(0);
            visiting[0] = true;
        }

        while let Some(curr) = queue.pop_front() {
            visiting[curr] = false;
            for (&to , &cost)  in table[curr].iter() {
                if reach[to] == -1 && disappear[to] > reach[curr] + cost {
                    reach[to] = reach[curr] + cost;
                    if !visiting[to] {
                        visiting[to] = true;
                        queue.push_back(to);
                    }
                    continue;
                }
                if reach[to] > reach[curr] + cost {
                    reach[to] = reach[curr] + cost;
                    if !visiting[to] {
                        visiting[to] = true;
                        queue.push_back(to);
                    }
                }
            }
        }

        reach
    }
}

#[derive(Debug)]
struct Edge {
    from: usize,
    to: usize,
    cost: i32,
}

pub struct Solution();

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0() {
        let edges = vec![vec![0, 1, 2], vec![1, 2, 1], vec![0, 2, 4]];
        let disappear = vec![1, 1, 5];
        assert_eq!(Solution::minimum_time(3, edges, disappear), vec![0, -1, 4]);
    }

    #[test]
    fn test_1() {
        let edges = vec![vec![0, 1, 2], vec![1, 2, 1], vec![0, 2, 4]];
        let disappear = vec![1, 3, 5];
        assert_eq!(Solution::minimum_time(3, edges, disappear), vec![0, 2, 3]);
    }

    #[test]
    fn test_2() {
        let edges = vec![vec![0, 1, 1]];
        let disappear = vec![1, 1];
        assert_eq!(Solution::minimum_time(2, edges, disappear), vec![0, -1]);
    }

    #[test]
    fn test_3() {
        let edges = vec![vec![0, 1, 1]];
        let disappear = vec![1, 1];
        assert_eq!(Solution::minimum_time(2, edges, disappear), vec![0, -1]);
    }
}
