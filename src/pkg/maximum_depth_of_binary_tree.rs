use crate::pkg::binary_tree_inorder_traversal::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let now_root = root.unwrap();
    let left_subtree = now_root.as_ref().borrow_mut().left.take();
    let right_subtree = now_root.as_ref().borrow_mut().right.take();

    i32::max(max_depth(left_subtree), max_depth(right_subtree)) + 1
}