use std::{cell::RefCell, rc::Rc};

type TreeNode = crate::common::TreeNode<i32>;

pub fn kth_largest_perfect_subtree(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut storage = Vec::<i32>::new();
    _ = subtree(root, &mut storage);
    storage.sort();
    if k > storage.len() as i32 {
        return -1;
    }
    let deep = storage[storage.len() - k as usize];
    (1 << deep) - 1
}

fn subtree(root: Option<Rc<RefCell<TreeNode>>>, storage: &mut Vec<i32>) -> Option<i32> {
    let Some(node) = root else {
        return Some(0);
    };

    let left = subtree(node.borrow().left.clone(), storage);
    let right = subtree(node.borrow().right.clone(), storage);
    let (Some(left), Some(right)) = (left, right) else {
        return None;
    };
    if left != right {
        return None;
    }
    let curr = left + 1;
    storage.push(curr);
    dbg!(format!("node: {}, curr: {}", node.borrow().val, curr));
    Some(curr)
}

#[cfg(test)]
mod tests {
    use crate::common::TreeNode;

    use super::*;

    #[test]
    fn test1() {
        let root = TreeNode::<i32>::from_zigzag_str("5,3,6,5,2,5,7,1,8,null,null,6,8");
        let k = 2;
        let result = kth_largest_perfect_subtree(root, k);
        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        let root = TreeNode::<i32>::from_zigzag_str("1,2,3,null,4");
        let k = 3;
        let result = kth_largest_perfect_subtree(root, k);
        assert_eq!(result, -1);
    }
}
