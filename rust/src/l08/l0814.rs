use std::{cell::RefCell, mem, ops::Deref, rc::Rc};
use crate::common; 

impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root;
        Self::prune_inner(&mut root);
        root
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

type TreeNode = common::TreeNode<i32>;

#[cfg(test)]
mod tests {
    use crate::common::TreeNode;

    use super::*;

    #[test]
    fn test() {
        let input = TreeNode::from_zigzag_str("1,null,0,0,1");
        let except = TreeNode::from_zigzag_str("1,null,0,null,1");

        let result = Solution::prune_tree(input);
        assert_eq!(except, result);
    }
}
