use crate::pkg::add_two_numbers::ListNode;

#[allow(dead_code)]
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