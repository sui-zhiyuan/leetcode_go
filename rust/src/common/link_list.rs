use std::fmt::{Debug, Formatter};

#[derive(PartialEq, Eq, Clone)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T>
where T:Copy
{
    pub fn new(val: T) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_slice(values: &[T]) -> Option<Box<ListNode<T>>> {
        let mut head = None;
        for v in values.iter().rev() {
            let mut node = ListNode::new(*v);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut res = Vec::new();
        let mut cur = Some(Box::new(self.clone()));
        while let Some(node) = cur {
            res.push(node.val);
            cur = node.next;
        }
        res
    }
}

// debug and test

impl Debug for ListNode<i32> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.to_vec())
    }
}