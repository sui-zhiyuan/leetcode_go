// use std::collections::HashSet;

// type ListNode = crate::common::ListNode<i32>;

// pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     let nums = nums.into_iter().collect::<HashSet<_>>();

//     let mut head = head;
//     while let Some(mut node) = head {
//         if nums.contains(&node.val) {
//             head = node.next;
//         } else {
//             break;
//         }
//     }

//     if head.is_none() {
//         return None;
//     }

//     let mut curr = &mut head.unwrap();
//     while let Some(mut node) = curr.next {
//         if nums.contains(&node.val) {
//             curr.next = node.next;
//             curr = &mut node;
//         } else {
//             curr = &mut node;
//         }
//     }
//     todo!()
// }