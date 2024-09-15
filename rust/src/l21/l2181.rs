type ListNode = crate::common::ListNode<i32>;

pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut t_head = Box::new(ListNode::new(0));

    let mut s_curr = &head;
    let mut value = Option::None;
    let mut t_curr = &mut t_head;

    while let Some(s_curr_inner) = s_curr.as_deref(){
        if s_curr_inner.val == 0 {
            if let Some(v) = value {
                let new_node = Box::new(ListNode::new(v));
                t_curr.next = Some(new_node);
                t_curr = t_curr.next.as_mut().unwrap();
                value = Option::None;
            }
            s_curr = &s_curr_inner.next;
            continue;
        }

        if value.is_none() {
            value = Some(s_curr_inner.val);
        } else {
            value = value.map(|v| v + s_curr_inner.val);
        }
        s_curr = &s_curr_inner.next;
    }

    if let Some(value) = value {
        let new_node = Box::new(ListNode::new(value));
        t_curr.next = Some(new_node);
    }

    t_head.next
}