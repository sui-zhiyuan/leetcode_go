use std::mem;

pub struct SegmentTree<T, F> {
    tree: Vec<T>,
    merge: F,

    // redundant data
    deep: usize,
    first_leaf: usize,
}

impl<T, F> SegmentTree<T, F>
where
    F: Fn(T, T) -> T,
    T: Default + Copy,
{
    pub fn new(data: Vec<T>, merge: F) -> Self {
        let (n_tree, deep, first_leaf) = Self::pre_calculate(data.len());
        let mut tree = SegmentTree {
            tree: vec![T::default(); n_tree],
            merge,
            deep,
            first_leaf,
        };
        tree.input(data);
        tree.build();
        tree
    }

    fn input(&mut self, data: Vec<T>) {
        for (i, v) in data.iter().enumerate() {
            let node = self.node(i);
            self.tree[node] = *v;
        }
    }

    fn build(&mut self) {
        let last_node = self.parent(self.tree.len());
        for i in (0..last_node).rev() {
            let left = self.left_child(i);
            let right = self.right_child(i);
            self.tree[i] = (self.merge)(self.tree[left], self.tree[right]);
        }
    }

    pub fn set_value(&mut self, i: usize, value: T) {
        self.change_value(i, |v| *v = value);
    }

    pub fn change_value(&mut self, i: usize, mut f: impl FnMut(&mut T)) {
        let mut node = self.node(i);
        f(&mut self.tree[node]);
        while node > 0 {
            node = self.parent(node);
            let left = self.left_child(node);
            let right = self.right_child(node);
            self.tree[node] = (self.merge)(self.tree[left], self.tree[right]);
        }
    }

    pub fn range(&self, left: usize, right: usize) -> T {
        assert!(left < right);
        assert!(right <= self.len());
        self.range_inner(0, left, right)
    }

    fn range_inner(&self, node: usize, left: usize, right: usize) -> T {
        let (node_left, node_right) = self.range_index(node);
        if node_left >= right || node_right <= left {
            return T::default();
        }
        if node_left >= left && node_right <= right {
            return self.tree[node];
        }
        let left_value = self.range_inner(self.left_child(node), left, right);
        let right_value = self.range_inner(self.right_child(node), left, right);
        (self.merge)(left_value, right_value)
    }
}

impl<T, F> SegmentTree<T, F>
where
    F: Fn(T, T) -> T,
    T: Default + Copy + Eq,
{
    pub fn new_empty(size: usize, merge: F) -> Self {
        let need_build = merge(T::default(), T::default()) == T::default();
        let (n_tree, deep, first_leaf) = Self::pre_calculate(size);
        let mut tree = SegmentTree {
            tree: vec![T::default(); n_tree],
            merge,
            deep,
            first_leaf,
        };

        if need_build {
            tree.build();
        }
        tree
    }

    pub fn values(&self) -> Vec<T> {
        (0..self.len()).map(|i| *self.value(i)).collect()
    }
}

impl<T, F> SegmentTree<T, F> {
    // data count
    pub fn len(&self) -> usize {
        self.tree.len().div_ceil(2)
    }

    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    pub fn value(&self, i: usize) -> &T {
        assert!(i < self.len());
        &self.tree[self.node(i)]
    }

    // For complete binary tree
    // NodeCount = LeafCount * 2 - 1 when all non-leaf nodes are full
    // NodeCount = 2 * LeafCount when there are 1 nodes without right child
    fn pre_calculate(n: usize) -> (usize, usize, usize) {
        if n == 0 {
            return (0, 0, 0);
        }

        let n_tree = 2 * n - 1;
        // floor(log2(n-1)) +2
        let deep = mem::size_of::<usize>() * 8 - (n - 1).leading_zeros() as usize + 1;
        // fist leaf of last level
        let first_leaf = (1 << (deep - 1)) - 1;
        (n_tree, deep, first_leaf)
    }

    fn is_leaf(&self, node: usize) -> bool {
        (2 * node + 1) >= self.tree.len()
    }

    fn left_child(&self, node: usize) -> usize {
        let left = 2 * node + 1;
        assert!(left < self.tree.len());
        left
    }

    fn right_child(&self, node: usize) -> usize {
        let right = 2 * node + 2;
        assert!(right < self.tree.len());
        right
    }

    fn parent(&self, node: usize) -> usize {
        let parent = (node - 1) / 2;
        assert!(parent < self.tree.len());
        parent
    }

    fn index(&self, node: usize) -> usize {
        assert!(self.is_leaf(node));
        assert!(node < self.tree.len());
        if node >= self.first_leaf {
            return node - self.first_leaf;
        }
        let last_leaf = self.tree.len();
        let last_leaf_parent = self.parent(last_leaf);
        last_leaf - self.first_leaf + node - last_leaf_parent
    }

    fn node(&self, index: usize) -> usize {
        assert!(index < self.len());
        let node = index + self.first_leaf;
        if node < self.tree.len() {
            return node;
        }
        let last_leaf = self.tree.len();
        let last_leaf_parent = self.parent(last_leaf);
        node - last_leaf + last_leaf_parent
    }

    fn range_index(&self, node: usize) -> (usize, usize) {
        // floor(log2(node +1)) 0 based depth
        let c_deep = mem::size_of::<usize>() * 8 - (node + 1).leading_zeros() as usize - 1;
        let mut change = self.deep - 1 - c_deep;
        if change == 0 {
            return (self.index(node), self.index(node) + 1);
        }

        // left = left * 2 + 1 for change times
        let mut left = (node << change) | ((1 << change) - 1);
        let mut right = node;
        if right & 1 == 1 {
            right = right * 2 + 2;
            change -= 1;
        }
        right = (right << change) | ((1 << (change + 1)) - 2);
        if left >= self.tree.len() {
            left = self.parent(left);
        }
        if right >= self.tree.len() {
            right = self.parent(right);
        }
        (self.index(left), self.index(right) + 1)
    }
}

// debug and test
use std::fmt::{Debug, Formatter, Result};

impl<T, F> Debug for SegmentTree<T, F>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut result = f.debug_struct("SegmentTree");
        result.field("tree", &self.tree);
        let data = (0..self.len())
            .map(|i| (i, self.value(i)))
            .collect::<Vec<_>>();
        result.field("data", &data);
        result.finish()
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Add;

    use super::*;

    #[test]
    fn test_segment_tree() {
        let input = 1..=100;
        let input = input.collect::<Vec<_>>();

        let mut st = SegmentTree::new(input.clone(), Add::add);
        assert_eq!(100, st.len());
        assert_eq!(199, st.tree.len());
        assert_eq!(5050, st.range(0, 100));
        assert_eq!(355, st.range(30, 40));

        st.set_value(30, 41);
        assert_eq!(5060, st.range(0, 100));
        assert_eq!(365, st.range(30, 40));
    }

    #[test]
    fn test_pre_calculate() {
        let (n_tree, deep, first_leaf) = SegmentTree::<u32, usize>::pre_calculate(24);
        assert_eq!(n_tree, 47);
        assert_eq!(deep, 6);
        assert_eq!(first_leaf, 31);
    }
}
