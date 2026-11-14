use crate::pkg::add_two_numbers::ListNode;

#[allow(dead_code)]
pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut pointer = &mut head;

    while list1.is_some() && list2.is_some() {
        if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
            pointer.as_mut().unwrap().next = list1;
            pointer = &mut pointer.as_mut().unwrap().next;
            list1 = pointer.as_mut().unwrap().next.take();
        } else {
            pointer.as_mut().unwrap().next = list2;
            pointer = &mut pointer.as_mut().unwrap().next;
            list2 = pointer.as_mut().unwrap().next.take();
        }
    }

    if list1.is_none() {
        pointer.as_mut().unwrap().next = list2;
    } else {
        pointer.as_mut().unwrap().next = list1;
    }

    head.unwrap().next
}