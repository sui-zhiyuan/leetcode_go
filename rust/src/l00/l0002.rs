use std::{cell::RefCell, rc::Rc};

type ListNode = crate::common::ListNode<i32>;

struct MutListNode {
    val: i32,
    next: Option<Rc<RefCell<MutListNode>>>,
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;

    let mut head = None;
    let mut tail = None;
    let mut carry = 0;
    loop {
        let v1 = if let Some(node) = l1 {
            l1 = node.next;
            node.val
        } else {
            0
        };

        let v2 = if let Some(node) = l2 {
            l2 = node.next;
            node.val
        } else {
            0
        };

        let sum = v1 + v2 + carry;
        carry = sum / 10;
        let sum = sum % 10;

        if sum == 0 && carry == 0 && l1.is_none() && l2.is_none() {
            break;
        }

        let node = MutListNode {
            val: sum,
            next: None,
        };
        match tail {
            None => {
                head = Some(Rc::new(RefCell::new(node)));
                tail.clone_from(&head);
            }
            Some(tail_node) => {
                let nv = Some(Rc::new(RefCell::new(node)));
                tail_node.borrow_mut().next.clone_from(&nv);
                tail = nv;
            }
        }
    }
    match copy(head) {
        None => Some(Box::new(ListNode::new(0))),
        Some(node) => Some(node),
    }
}

fn copy(head: Option<Rc<RefCell<MutListNode>>>) -> Option<Box<ListNode>> {
    match head {
        None => None,
        Some(node) => {
            let v = ListNode {
                val: node.borrow().val,
                next: copy(node.borrow().next.clone()),
            };
            Some(Box::new(v))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let l1 = ListNode::from_slice(&[2, 4, 3]);
        let l2 = ListNode::from_slice(&[5, 6, 4]);
        let l3 = ListNode::from_slice(&[7, 0, 8]);
        assert_eq!(l3, add_two_numbers(l1, l2));
    }

    #[test]
    fn test2() {
        let l1 = ListNode::from_slice(&[0]);
        let l2 = ListNode::from_slice(&[0]);
        let l3 = ListNode::from_slice(&[0]);
        assert_eq!(l3, add_two_numbers(l1, l2));
    }

    #[test]
    fn test3() {
        let l1 = ListNode::from_slice(&[9, 9, 9, 9, 9, 9, 9]);
        let l2 = ListNode::from_slice(&[9, 9, 9, 9]);
        let l3 = ListNode::from_slice(&[8, 9, 9, 9, 0, 0, 0, 1]);
        assert_eq!(l3, add_two_numbers(l1, l2));
    }
}