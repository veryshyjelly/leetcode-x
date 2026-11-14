use crate::pkg::add_two_numbers::ListNode;

#[allow(dead_code)]
pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut next_group = head;

    let mut master = Some(Box::new(ListNode::new(0)));
    let mut pointer = &mut master;

    'outer: while next_group.is_some() {
        let mut cutter = &mut next_group;
        for _ in 0..k - 1 {
            if cutter.as_ref().unwrap().next.is_none() {
                pointer.as_mut().unwrap().next = next_group;
                break 'outer;
            }
            cutter = &mut cutter.as_mut().unwrap().next;
        }
        let rx = cutter.as_mut().unwrap().next.take();
        let head = next_group.take();
        pointer.as_mut().unwrap().next = reverse_list(head);
        for _ in 0..k {
            pointer = &mut pointer.as_mut().unwrap().next;
        }
        next_group = rx;
    }

    master.unwrap().next
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut hook = (head, None);

    while hook.0.is_some() {
        let mut disassociate = hook.0;
        hook.0 = disassociate.as_mut().unwrap().next.take();
        disassociate.as_mut().unwrap().next = hook.1;
        hook.1 = disassociate;
    }

    hook.1
}