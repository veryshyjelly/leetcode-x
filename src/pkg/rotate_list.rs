use crate::pkg::add_two_numbers::ListNode;

#[allow(dead_code)]
pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    let mut n = 1;
    let mut pointer = &mut head;
    while pointer.as_ref().unwrap().next.is_some() {
        pointer = &mut pointer.as_mut().unwrap().next;
        n += 1;
    }

    let k = (k as usize) % n;
    if k == 0 {
        return head;
    } else {
        let mut node_ref = &mut head;
        for _ in k..n {
            node_ref = &mut node_ref.as_mut().unwrap().next;
        }
        let mut new_head = node_ref.as_mut().unwrap().next.take();
        node_ref = &mut new_head;
        while node_ref.as_ref().unwrap().next.is_some() {
            node_ref = &mut node_ref.as_mut().unwrap().next;
        }
        node_ref.as_mut().unwrap().next = head;

        new_head
    }
}