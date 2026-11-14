use crate::pkg::add_two_numbers::ListNode;

#[allow(dead_code)]
pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    let mut torch = &head.clone();

    for _ in 0..n {
        if torch.is_none() {
            break;
        }
        torch = &torch.as_ref().unwrap().next;
    }

    if torch.is_none() {
        return head?.next;
    }

    let mut cutter = &mut head;
    while torch.as_ref().unwrap().next.is_some() {
        cutter = &mut cutter.as_mut().unwrap().next;
        torch = &torch.as_ref().unwrap().next;
    }

    if cutter.as_ref().unwrap().next.is_none() {
        return head;
    }

    cutter.as_mut().unwrap().next = cutter.as_ref()?.next.as_ref()?.next.clone();

    head
}