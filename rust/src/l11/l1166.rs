use std::collections::HashMap;

pub struct FileSystem {
    /// node[0] is root node
    nodes: Vec<Node>,
}

const ROOT_NODE: usize = 0;

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            nodes: vec![Node {
                children: HashMap::new(),
                value: 0,
            }],
        }
    }

    pub fn create_path(&mut self, path: String, value: i32) -> bool {
        let path = Self::get_path_part(&path);
        let Some(parent_node) = self.find_node(ROOT_NODE, &path[..(path.len() - 1)]) else {
            return false;
        };

        let last_part = *path.last().unwrap();
        if self.nodes[parent_node].children.contains_key(last_part) {
            return false;
        }

        self.nodes.push(Node {
            children: HashMap::new(),
            value,
        });

        let last_node = self.nodes.len() - 1;
        self.nodes[parent_node]
            .children
            .insert(last_part.to_string(), last_node);

        true
    }

    pub fn get(&self, path: String) -> i32 {
        let path = Self::get_path_part(&path);
        let Some(curr_node) = self.find_node(ROOT_NODE, &path) else {
            return -1;
        };

        self.nodes[curr_node].value
    }
}

struct Node {
    children: HashMap<String, usize>,
    value: i32,
}

impl FileSystem {
    fn find_node(&self, root: usize, path: &[&str]) -> Option<usize> {
        let mut curr = root;

        for step in path {
            let Some(v) = self.nodes[curr].children.get(*step) else {
                return None;
            };
            curr = *v;
        }

        Some(curr)
    }

    fn get_path_part(path: &String) -> Vec<&str> {
        let path = path.trim_matches('/').split('/').collect::<Vec<_>>();
        assert!(path.len() > 0, "path is empty");
        path
    }
}
