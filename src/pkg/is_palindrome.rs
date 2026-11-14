use crate::pkg::add_two_numbers::ListNode;

#[allow(dead_code)]
pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
    let (mut tortoise, mut hare) = (head.clone(), head.clone());
    while hare.as_ref().unwrap().next.is_some()
        && hare.as_ref().unwrap().next.as_ref().unwrap().next.is_some()
    {
        tortoise = tortoise.unwrap().next;
        hare = hare.unwrap().next.unwrap().next;
    }
    let mut reversed_tortoise = reverse_list(tortoise.unwrap().next);
    while reversed_tortoise.is_some() {
        if head.as_ref().unwrap().val != reversed_tortoise.as_ref().unwrap().val {
            return false;
        }
        reversed_tortoise = reversed_tortoise.unwrap().next;
        head = head.unwrap().next;
    }
    true
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