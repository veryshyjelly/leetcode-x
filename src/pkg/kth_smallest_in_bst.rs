use crate::pkg::binary_tree_inorder_traversal::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
    kth_smallest_helper(root, &mut k)
}

pub fn kth_smallest_helper(root: Option<Rc<RefCell<TreeNode>>>, k: &mut i32) -> i32 {
    if root.is_none() {
        return -1;
    }
    let now_root = root.unwrap();
    let left_subtree = now_root.borrow_mut().left.take();
    let right_subtree = now_root.borrow_mut().right.take();
    let left_answer = kth_smallest_helper(left_subtree, k);
    if *k <= 0 {
        left_answer
    } else if *k == 1 {
        *k = 0;
        now_root.borrow().val
    } else {
        *k = *k - 1;
        kth_smallest_helper(right_subtree, k)
    }
}