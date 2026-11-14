use crate::pkg::add_two_numbers::ListNode;

#[allow(dead_code)]
pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut counter = &head;
    let mut n = 0;
    while counter.is_some() {
        counter = &counter.as_ref().unwrap().next;
        n += 1;
    }
    n = n / 2;
    for _ in 0..n {
        head = head.unwrap().next;
    }
    head
}