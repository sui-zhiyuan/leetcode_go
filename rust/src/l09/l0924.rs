use std::collections::HashMap;

impl Solution {
    pub fn min_malware_spread(graph: Vec<Vec<i32>>, initial: Vec<i32>) -> i32 {
        let n = graph.len();
        let mut dsu = DisjointSet::new(n);
        for (i, row) in graph.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate().take(i) {
                if cell == 1 {
                    dsu.union(i, j);
                }
            }
        }

        let init_effect = initial
            .iter()
            .copied()
            .map(|v| {
                let v: usize = v.try_into().unwrap();
                (v, dsu.find(v), dsu.size(v))
            })
            .collect::<Vec<_>>();

        let root_effect = init_effect
            .iter()
            .fold(HashMap::new(), |mut map, (_, root, _)| {
                *map.entry(root).or_insert(0) += 1;
                map
            });

        let mut effect = 0;
        let mut result: usize = initial[0].try_into().unwrap();
        for &(node, root, size) in init_effect.iter() {
            let size = match root_effect.get(&root).unwrap() {
                0 => unreachable!(),
                1 => size,
                _ => 0,
            };

            if size > effect || (size == effect && node < result) {
                effect = size;
                result = node;
            }
        }

        result.try_into().unwrap()
    }
}

#[derive(Debug)]
struct DisjointSet {
    nodes: Vec<Node>,
}

impl DisjointSet {
    fn new(count: usize) -> Self {
        Self {
            nodes: vec![Node::Root { size: 1 }; count],
        }
    }

    fn find(&mut self, node: usize) -> usize {
        let mut path = Vec::new();
        let mut node = node;
        while let Node::Child { parent } = self.nodes[node] {
            path.push(node);
            node = parent;
        }
        for child in path {
            self.nodes[child] = Node::Child { parent: node };
        }
        node
    }

    fn union(&mut self, left: usize, right: usize) {
        let left = self.find(left);
        let right = self.find(right);
        if left == right {
            return;
        }

        let left_count = self.nodes[left].count();

        let right_count = self.nodes[right].count();

        if left_count < right_count {
            self.nodes[left] = Node::Child { parent: right };
            self.nodes[right] = Node::Root {
                size: left_count + right_count,
            };
        } else {
            self.nodes[right] = Node::Child { parent: left };
            self.nodes[left] = Node::Root {
                size: left_count + right_count,
            };
        }
    }

    // fn same(&mut self, left: usize, right: usize) -> bool {
    //     self.find(left) == self.find(right)
    // }

    fn size(&mut self, node: usize) -> usize {
        let root = self.find(node);
        self.nodes[root].count()
    }
}

#[derive(Clone, Copy, Debug)]
enum Node {
    Root { size: usize },
    Child { parent: usize },
}

impl Node {
    /// count should not be called for non-root nodes
    fn count(&self) -> usize {
        match self {
            Node::Root { size } => *size,
            _ => unreachable!(),
        }
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
