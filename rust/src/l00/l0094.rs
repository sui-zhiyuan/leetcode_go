use std::cell::RefCell;
use std::rc::Rc;

type TreeNode = crate::common::TreeNode<i32>;

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut stack = Vec::new();
    stack.push((State::Start, root.clone()));

    let mut result = Vec::new();
    while let Some(curr) = stack.last_mut() {
        let mut call = None;
        match curr {
            (State::Start, None) => {
                stack.pop();
            }
            (s @ State::Start, Some(node)) => {
                *s = State::CalledLeft;
                call = Some(node.borrow().left.clone());
            }
            (s @ State::CalledLeft, Some(node)) => {
                let node = node.borrow();
                result.push(node.val);
                *s = State::CalledRight;
                call = Some(node.right.clone());
            }
            (State::CalledRight, Some(_)) => {
                stack.pop();
            }
            _ => unreachable!("{:?}", curr),
        }

        if let Some(call) = call {
            stack.push((State::Start, call));
        }
    }
    result
}

#[derive(Debug, Clone)]
enum State {
    Start,
    CalledLeft,
    CalledRight,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameter_tests;

    struct Case {
        input: &'static str,
        expect: Vec<i32>,
    }

    fn test(c: Case) {
        let root = TreeNode::from_zigzag_str(c.input);
        assert_eq!(c.expect, inorder_traversal(root));
    }

    parameter_tests! {
        test,
        (case1: Case {
            input: "[1,null,2,3]",
            expect: vec![1,3,2],
        }),
        (case2: Case {
            input: "[]",
            expect: vec![],
        }),
        (case3: Case {
            input: "[1]",
            expect: vec![1],
        }),
    }
}
