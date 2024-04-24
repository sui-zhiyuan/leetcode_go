use std::{cell::RefCell, cmp, rc::Rc};

type TreeNode = crate::common::TreeNode<i32>;

pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
    let result = dfs(root, start);
    result.max_infect_path
}

fn dfs(node: Option<Rc<RefCell<TreeNode>>>, start: i32) -> Result {
    let node = match node {
        Some(node) => node,
        None => {
            return Result {
                max_path: 0,
                infect_path: None,
                max_infect_path: 0,
            }
        }
    };
    let left = node.borrow().left.clone();
    let left = dfs(left, start);
    let right = node.borrow().right.clone();
    let right = dfs(right, start);

    if node.borrow().val == start {
        Result {
            max_path: cmp::max(left.max_path, right.max_path) +1,
            infect_path: Some(0),
            max_infect_path: cmp::max(left.max_path, right.max_path),
        }
    }else if let Some(infect_path) = left.infect_path {
        Result {
            max_path: cmp::max(left.max_path, right.max_path) +1,
            infect_path: Some(infect_path + 1),
            max_infect_path: cmp::max(left.max_infect_path, infect_path + 1 + right.max_path),
        }
    } else if let Some(infect_path) = right.infect_path {
        Result {
            max_path: cmp::max(left.max_path, right.max_path) +1,
            infect_path: Some(infect_path + 1),
            max_infect_path: cmp::max(right.max_infect_path, infect_path + 1 + left.max_path),
        }
    } else {
        Result {
            max_path: cmp::max(left.max_path, right.max_path) +1,
            infect_path: None,
            max_infect_path: cmp::max(left.max_infect_path, right.max_infect_path),
        }
    }
}

struct Result {
    max_path: i32,
    infect_path: Option<i32>,
    max_infect_path: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root = TreeNode::from_zigzag_str("1,5,3,null,4,10,6,9,2");
        let start = 3;
        let result = 4;
        assert_eq!(amount_of_time(root, start), result);
    }

    #[test]
    fn test_2() {
        let root = TreeNode::from_zigzag_str("1");
        let start = 1;
        let result = 0;
        assert_eq!(amount_of_time(root, start), result);
    }
}