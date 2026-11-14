use crate::pkg::binary_tree_inorder_traversal::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return None;
    }

    let now_root = root.unwrap();
    let now_val = now_root.borrow().val;
    if now_val == val {
        Some(now_root)
    } else if now_val > val {
        search_bst(now_root.borrow_mut().left.take(), val)
    } else {
        search_bst(now_root.borrow_mut().right.take(), val)
    }
}