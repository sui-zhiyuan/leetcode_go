use std::collections::BinaryHeap;

pub fn minimum_time(n: i32, edges: Vec<Vec<i32>>, disappear: Vec<i32>) -> Vec<i32> {
    let n = n as usize;

    let mut n_edges = vec![Vec::<(usize, i32)>::new(); n];
    for e in edges {
        n_edges[e[0] as usize].push((e[1] as usize, e[2]));
        n_edges[e[1] as usize].push((e[0] as usize, e[2]));
    }

    let mut result = vec![-1; n];
    result[0] = 0;
    let mut queue = BinaryHeap::new();
    queue.push(Node { index: 0, time: 0 });
    while let Some(Node {
        index: curr,
        time,
    }) = queue.pop()
    {
        if result[curr] < time {
            continue;
        }
        let c_edges = &n_edges[curr];
        for &(next, time) in c_edges {
            if disappear[next] <= result[curr] + time {
                continue;
            }

            if result[next] == -1 || result[next] > result[curr] + time {
                result[next] = result[curr] + time;
                queue.push(Node {
                    index: next,
                    time: result[next],
                });
            }
        }
    }

    result
}

struct Node {
    index: usize,
    time: i32,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time).reverse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 3;
        let edges = [[0, 1, 2], [1, 2, 1], [0, 2, 4]];
        let edges = edges.iter().map(|r| r.to_vec()).collect();
        let disappear = vec![1, 1, 5];
        let result = minimum_time(n, edges, disappear);
        assert_eq!(result, vec![0, -1, 4]);
    }
}
