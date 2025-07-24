pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
    let edges = edges
        .iter()
        .map(|v| {
            assert_eq!(2, v.len());
            [v[0] as usize, v[1] as usize]
        })
        .collect::<Vec<_>>();
    let tree = Tree::new(nums, edges);

    // remove edge for node to parent

    let mut result = i32::MAX;
    for first_node in 1..tree.nodes.len() {
        for second_nodes in first_node + 1..tree.nodes.len() {
            let sorted = match tree.check_ancestor(first_node, second_nodes) {
                AncestorResult::Left => Some((first_node, second_nodes)),
                AncestorResult::Right => Some((second_nodes, first_node)),
                AncestorResult::NoneRelated => None,
            };

            let (v1, v2, v3) = match sorted {
                Some((ancestor, successor)) => {
                    let v1 =
                        tree.nodes[0].sub_tree_xor_value ^ tree.nodes[ancestor].sub_tree_xor_value;
                    let v2 = tree.nodes[ancestor].sub_tree_xor_value
                        ^ tree.nodes[successor].sub_tree_xor_value;
                    let v3 = tree.nodes[successor].sub_tree_xor_value;
                    (v1, v2, v3)
                }
                None => {
                    let v1 = tree.nodes[0].sub_tree_xor_value
                        ^ tree.nodes[first_node].sub_tree_xor_value
                        ^ tree.nodes[second_nodes].sub_tree_xor_value;
                    let v2 = tree.nodes[first_node].sub_tree_xor_value;
                    let v3 = tree.nodes[second_nodes].sub_tree_xor_value;
                    (v1, v2, v3)
                }
            };
            
            result = result.min(max_sub_value(v1, v2, v3))
        }
    }

    result
}

struct Tree {
    // always use 0 node as root;
    nodes: Vec<Node>,
}

#[derive(Clone)]
struct Node {
    value: i32,
    sub_tree_xor_value: i32,
    children: Vec<usize>,
    euler_seq: (i32, i32),
}

impl Tree {
    fn new(nums: Vec<i32>, edges: Vec<[usize; 2]>) -> Self {
        let mut edge_table = vec![Vec::<usize>::new(); nums.len()];

        for [l, r] in edges {
            edge_table[l].push(r);
            edge_table[r].push(l);
        }

        let mut tree = Tree {
            nodes: vec![
                Node {
                    value: 0,
                    children: Vec::new(),
                    sub_tree_xor_value: 0,
                    euler_seq: (0, 0),
                };
                nums.len()
            ],
        };

        for (i, n) in nums.into_iter().enumerate() {
            tree.nodes[i].value = n;
        }

        tree.fill(&edge_table, 0, None, &mut 0);

        tree
    }

    fn fill(
        &mut self,
        edge_table: &[Vec<usize>],
        curr: usize,
        parent: Option<usize>,
        euler_seq: &mut i32,
    ) {
        let mut children = edge_table[curr]
            .iter()
            .copied()
            .filter(|&v| Some(v) != parent)
            .collect::<Vec<_>>();

        children.sort();

        *euler_seq += 1;
        self.nodes[curr].euler_seq.0 = *euler_seq;

        let mut xor_value = self.nodes[curr].value;
        for &child in children.iter() {
            self.fill(edge_table, child, Some(curr), euler_seq);
            xor_value ^= self.nodes[child].sub_tree_xor_value;
        }

        self.nodes[curr].sub_tree_xor_value = xor_value;
        self.nodes[curr].children = children;
        self.nodes[curr].euler_seq.1 = *euler_seq;
    }

    // 欧拉序判断节点关系
    fn check_ancestor(&self, left: usize, right: usize) -> AncestorResult {
        let left = &self.nodes[left];
        let right = &self.nodes[right];

        if left.euler_seq.0 <= right.euler_seq.0 && left.euler_seq.1 >= right.euler_seq.1 {
            return AncestorResult::Left;
        }

        if right.euler_seq.0 <= left.euler_seq.0 && right.euler_seq.1 >= left.euler_seq.1 {
            return AncestorResult::Right;
        }

        AncestorResult::NoneRelated
    }
}

enum AncestorResult {
    Left,  // left is Ancestor
    Right, // right is Ancestor
    NoneRelated,
}

fn max_sub_value(v1: i32, v2: i32, v3: i32) -> i32 {
    let max = v1.max(v2).max(v3);
    let min = v1.min(v2).min(v3);

    max - min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 5, 5, 4, 11];
        let edges = vec![[0, 1], [1, 2], [1, 3], [3, 4]]
            .iter()
            .map(|v| v.to_vec())
            .collect::<Vec<_>>();

        assert_eq!(9, minimum_score(nums, edges))
    }
}
