use std::{cell::RefCell, collections::LinkedList, mem, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
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
            panic!("missing root node");
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
            if nums.len() < 2 {
                panic!("Missing nodes")
            }
            let left = nums[0].take().and_then(|v| Self::new_node(v, None, None));
            let right = nums[1].take().and_then(|v| Self::new_node(v, None, None));
            nums = &mut nums[2..];
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
        queue.push_back(root);

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();

            if let Some(inner) = &node {
                let inner = inner.borrow();
                queue.push_back(inner.left.clone());
                queue.push_back(inner.right.clone());
            }

            result.push(node.map(&mut f));
        }
        result
    }
}
