use std::{
    cell::RefCell,
    collections::LinkedList,
    fmt::{self, Debug, Formatter},
    mem,
    rc::Rc,
};

#[derive(PartialEq, Eq, Clone)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T> {
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn new_node(
        val: T,
        left: Option<Rc<RefCell<Self>>>,
        right: Option<Rc<RefCell<Self>>>,
    ) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    pub fn from_zigzag(nums: Vec<Option<T>>) -> Option<Rc<RefCell<Self>>> {
        let mut nums = nums;
        if nums.is_empty() {
            return None;
        }
        let root = nums[0].take().and_then(|v| Self::new_node(v, None, None));
        let mut nums = &mut nums[1..];
        let mut queue = LinkedList::new();
        match root {
            Some(ref root) => {
                queue.push_back(root.clone());
            }
            None => return None,
        }

        while !queue.is_empty() {
            let parent = queue.pop_front().unwrap();
            let mut left = None;
            let mut right = None;
            let mut moved = 0;
            if !nums.is_empty() {
                left = nums[0].take().and_then(|v| Self::new_node(v, None, None));
                moved = 1;
            }
            if nums.len() >= 2 {
                right = nums[1].take().and_then(|v| Self::new_node(v, None, None));
                moved = 2;
            }
            nums = &mut nums[moved..];
            if let Some(left) = left {
                parent.borrow_mut().left = Some(left.clone());
                queue.push_back(left);
            }
            if let Some(right) = right {
                parent.borrow_mut().right = Some(right.clone());
                queue.push_back(right);
            }
        }
        root
    }

    /// This method is unsafe. Call this function will change all node value to Default.
    /// But Rc can be cloned without cloning the inner value. Use a clone of root after
    /// calling this method will lead to a bug. Use to_zigzag_clone instead if root needed
    /// after calling this method.
    pub fn into_zigzag(root: Option<Rc<RefCell<Self>>>) -> Vec<Option<T>>
    where
        T: Default,
    {
        Self::to_zigzag_inner(root, |node| {
            let mut node = node.borrow_mut();
            mem::take(&mut node.val)
        })
    }

    pub fn to_zigzag_clone(root: &Option<Rc<RefCell<Self>>>) -> Vec<Option<T>>
    where
        T: Clone,
    {
        Self::to_zigzag_inner(root.clone(), |node| {
            let node = node.borrow();
            node.val.clone()
        })
    }

    fn to_zigzag_inner<U, F>(root: Option<Rc<RefCell<Self>>>, mut f: F) -> Vec<Option<U>>
    where
        F: FnMut(Rc<RefCell<Self>>) -> U,
    {
        let mut queue = LinkedList::new();
        let mut result = Vec::new();
        let mut available_node = 0;
        available_node += Self::queue_push(&mut queue, root);

        while available_node > 0 {
            let (node, count) = Self::queue_pop(&mut queue);
            available_node -= count;

            if let Some(inner) = &node {
                let inner = inner.borrow();
                available_node += Self::queue_push(&mut queue, inner.left.clone());
                available_node += Self::queue_push(&mut queue, inner.right.clone());
            }

            result.push(node.map(&mut f));
        }
        result
    }

    fn queue_push(
        queue: &mut LinkedList<Option<Rc<RefCell<Self>>>>,
        node: Option<Rc<RefCell<Self>>>,
    ) -> usize {
        let count = match node {
            Some(_) => 1,
            None => 0,
        };
        queue.push_back(node);
        count
    }

    fn queue_pop(
        queue: &mut LinkedList<Option<Rc<RefCell<Self>>>>,
    ) -> (Option<Rc<RefCell<Self>>>, usize) {
        let node = queue.pop_front().unwrap();
        let count = match &node {
            Some(_) => 1,
            None => 0,
        };
        (node, count)
    }
}

impl<T> Debug for TreeNode<T>
where
    T: Debug + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let root = Some(Rc::new(RefCell::new(self.clone())));
        let view = Self::to_zigzag_clone(&root);
        write!(f, "{:?}", view)
    }
}

impl<T> TreeNode<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    pub fn from_zigzag_str(input: &str) -> Option<Rc<RefCell<Self>>> {
        let nums = parse_list(input);
        Self::from_zigzag(nums)
    }
}

pub fn parse_list<T>(input: &str) -> Vec<Option<T>>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    if input == "[]" {
        return vec![None];
    }

    input
        .trim()
        .strip_prefix("[")
        .expect("NoPrefixOfSlice")
        .strip_suffix("]")
        .expect("NoAppendixOfSlice")
        .split(',')
        .map(|s| {
            let s = s.trim();
            if s == "null" {
                None
            } else {
                Some(s.parse().unwrap())
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_node() {
        let nums = vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5)];

        let n20 = TreeNode::new_node(4, None, None);
        let n21 = TreeNode::new_node(5, None, None);
        let n10 = TreeNode::new_node(2, None, None);
        let n11 = TreeNode::new_node(3, n20, n21);
        let except_root = TreeNode::new_node(1, n10, n11);

        let root = TreeNode::from_zigzag(nums.clone());
        assert_eq!(except_root, root);
        let view = TreeNode::to_zigzag_clone(&root);
        assert!(nums == view);
    }

    #[test]
    fn test_parse_list() {
        let input = "1, 2, 3, null, null, 4, 5";
        let nums = vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5)];
        assert_eq!(nums, parse_list(input));
    }
}
