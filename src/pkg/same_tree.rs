use crate::pkg::binary_tree_inorder_traversal::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if p.is_none() || q.is_none() {
        return p.is_none() && q.is_none();
    }

    let first = p.unwrap();
    let second = q.unwrap();

    if first.as_ref().borrow().val != second.as_ref().borrow().val {
        return false;
    }

    let first_left = first.as_ref().borrow_mut().left.take();
    let first_right = first.as_ref().borrow_mut().right.take();
    let second_left = second.as_ref().borrow_mut().left.take();
    let second_right = second.as_ref().borrow_mut().right.take();

    is_same_tree(first_left, second_left) && is_same_tree(first_right, second_right)
}