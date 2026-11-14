use crate::pkg::binary_tree_inorder_traversal::TreeNode;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

#[allow(dead_code)]
pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
    let mut set = HashSet::new();
    find_target_helper(root, &mut set, k)
}

pub fn find_target_helper(
    root: Option<Rc<RefCell<TreeNode>>>,
    targets: &mut HashSet<i32>,
    k: i32,
) -> bool {
    if root.is_none() {
        return false;
    }
    let now_root = root.unwrap();
    let left_subtree = now_root.borrow_mut().left.take();
    let right_subtree = now_root.borrow_mut().right.take();
    if targets.contains(&now_root.borrow().val) {
        return true;
    }
    targets.insert(k - now_root.borrow().val);
    if find_target_helper(left_subtree, targets, k) {
        return true;
    }
    find_target_helper(right_subtree, targets, k)
}