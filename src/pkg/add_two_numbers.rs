#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut carry = 0;
    let mut new_list = ListNode::new(0);
    let mut pointer = &mut new_list;

    let (mut l1, mut l2) = (l1, l2);

    while l1.is_some() && l2.is_some() {
        let (first, second) = (l1.as_ref().unwrap(), l2.as_ref().unwrap());
        pointer.next = Some(Box::new(ListNode::new(
            (first.val + second.val + carry) % 10,
        )));
        carry = (first.val + second.val + carry) / 10;
        pointer = pointer.next.as_mut().unwrap();
        l1 = l1.unwrap().next;
        l2 = l2.unwrap().next;
    }

    while l1.is_some() {
        let first = l1.as_ref().unwrap();
        pointer.next = Some(Box::new(ListNode::new((first.val + carry) % 10)));
        carry = (first.val + carry) / 10;
        pointer = pointer.next.as_mut().unwrap();
        l1 = l1.unwrap().next;
    }

    while l2.is_some() {
        let second = l2.as_ref().unwrap();
        pointer.next = Some(Box::new(ListNode::new((second.val + carry) % 10)));
        carry = (second.val + carry) / 10;
        pointer = pointer.next.as_mut().unwrap();
        l2 = l2.unwrap().next;
    }

    while carry != 0 {
        pointer.next = Some(Box::new(ListNode::new(carry % 10)));
        carry = carry / 10;
        pointer = pointer.next.as_mut().unwrap();
    }

    new_list.next
}