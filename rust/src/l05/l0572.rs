use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::common::TreeNode<i32>;

pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_subtree_inner(&root, &sub_root)
}

pub fn is_subtree_inner(
    root: &Option<Rc<RefCell<TreeNode>>>,
    sub_root: &Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    if is_equal(root, sub_root) {
        return true;
    }

    let Some(root) = root else {
        return false;
    };

    let left = root.borrow().left.clone();
    if is_subtree_inner(&left, sub_root) {
        return true;
    }

    let right = root.borrow().right.clone();
    if is_subtree_inner(&right, sub_root) {
        return true;
    }

    false
}

fn is_equal(
    node_a: &Option<Rc<RefCell<TreeNode>>>,
    node_b: &Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match (node_a, node_b) {
        (None, None) => true,
        (Some(node_a), Some(node_b)) => {
            let node_a = node_a.borrow();
            let node_b = node_b.borrow();
            node_a.val == node_b.val
                && is_equal(&node_a.left, &node_b.left)
                && is_equal(&node_a.right, &node_b.right)
        }
        _ => false,
    }
}
