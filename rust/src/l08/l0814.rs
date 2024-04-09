use std::{cell::RefCell, mem, ops::Deref, rc::Rc};

impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root;
        Self::prune_inner(&mut root);
        return root;
    }

    fn prune_inner(curr: &mut Option<Rc<RefCell<TreeNode>>>) {
        let c = match curr.deref() {
            Some(root) => root,
            None => return,
        };
        let mut c = c.try_borrow_mut().unwrap();
        Self::prune_inner(&mut c.left);
        Self::prune_inner(&mut c.right);
        if c.left.is_some() || c.right.is_some() || c.val != 0 {
            return;
        }
        mem::drop(c);
        *curr = None;
    }
}

pub struct Solution();

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn new_node(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode { val, left, right }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let p31 = TreeNode::new_node(0, None, None);
        let p32 = TreeNode::new_node(1, None, None);
        let p21 = TreeNode::new_node(0, Some(p31), Some(p32));
        let root = TreeNode::new_node(1, None, Some(p21));

        let r31 = TreeNode::new_node(1, None, None);
        let r21 = TreeNode::new_node(0, None, Some(r31));
        let except = TreeNode::new_node(1, None, Some(r21));

        let result = Solution::prune_tree(Some(root));
        assert_eq!(Some(except), result);
    }
}
