use crate::pkg::binary_tree_inorder_traversal::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    valid_bst_helper(root, i64::MAX, i64::MIN)
}

pub fn valid_bst_helper(root: Option<Rc<RefCell<TreeNode>>>, left: i64, right: i64) -> bool {
    if root.is_none() {
        return true;
    }

    let now_root = root.unwrap();
    let now_value = now_root.as_ref().borrow().val;

    if now_value as i64 >= left || now_value as i64 <= right {
        return false;
    }

    let left_subtree = now_root.as_ref().borrow_mut().left.take();
    let right_subtree = now_root.as_ref().borrow_mut().right.take();

    valid_bst_helper(left_subtree, now_value as i64, right)
        && valid_bst_helper(right_subtree, left, now_value as i64)
}