use std::fmt::Debug;

#[derive(Clone)]
pub struct DisjointSet<T, F> {
    nodes: Vec<Node<T>>,
    f: F,
}

impl<T, F> DisjointSet<T, F> {
    pub fn find(&mut self, node: usize) -> usize {
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

    pub fn same(&mut self, left: usize, right: usize) -> bool {
        self.find(left) == self.find(right)
    }

    pub fn size(&mut self, node: usize) -> usize {
        let root = self.find(node);
        self.nodes[root].count()
    }
}

impl<T, F> DisjointSet<T, F>
where
    F: FnMut(T, T) -> T,
    T: Copy,
{
    pub fn new(value: &[T], f: F) -> Self {
        Self {
            nodes: value
                .iter()
                .map(|&value| Node::Root { size: 1, value })
                .collect(),
            f,
        }
    }

    pub fn add(&mut self , value: T) -> usize {
        let index = self.nodes.len();
        self.nodes.push(Node::Root { size: 1, value });
        index
    }

    pub fn union(&mut self, left: usize, right: usize) {
        let left = self.find(left);
        let right = self.find(right);
        if left == right {
            return;
        }

        let left_count = self.nodes[left].count();
        let right_count = self.nodes[right].count();
        let merged_value = (self.f)(self.nodes[left].value(), self.nodes[right].value());

        if left_count < right_count {
            self.nodes[left] = Node::Child { parent: right };
            self.nodes[right] = Node::Root {
                size: left_count + right_count,
                value: merged_value,
            };
        } else {
            self.nodes[right] = Node::Child { parent: left };
            self.nodes[left] = Node::Root {
                size: left_count + right_count,
                value: merged_value,
            };
        }
    }

    pub fn value(&mut self, node: usize) -> T {
        let root = self.find(node);
        self.nodes[root].value()
    }
}

#[derive(Debug, Clone)]
enum Node<T> {
    Root { size: usize, value: T },
    Child { parent: usize },
}

impl<T> Node<T> {
    /// count should not be called for non-root nodes
    fn count(&self) -> usize {
        match self {
            Node::Root { size, .. } => *size,
            _ => unreachable!(),
        }
    }
}

impl<T> Node<T>
where
    T: Copy,
{
    /// count should not be called for non-root nodes
    fn value(&self) -> T {
        match self {
            Node::Root { value, .. } => *value,
            _ => unreachable!(),
        }
    }
}

// debug and test
impl<T, F> Debug for DisjointSet<T, F>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DisjointSet")
            .field("nodes", &self.nodes)
            .finish()
    }
}
