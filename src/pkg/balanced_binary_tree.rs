use crate::pkg::binary_tree_inorder_traversal::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_balanced_helper(root).0
}

pub fn is_balanced_helper(root: Option<Rc<RefCell<TreeNode>>>) -> (bool, usize) {
    if root.is_none() {
        return (true, 0);
    }

    let now_root = root.unwrap();
    let left_subtree = is_balanced_helper(now_root.as_ref().borrow_mut().left.take());

    if !left_subtree.0 {
        return left_subtree;
    }

    let right_subtree = is_balanced_helper(now_root.as_ref().borrow_mut().right.take());
    if !right_subtree.0 {
        return right_subtree;
    }

    if usize::max(left_subtree.1, right_subtree.1) - usize::min(left_subtree.1, right_subtree.1) > 1
    {
        return (false, 0);
    }

    return (true, usize::max(left_subtree.1, right_subtree.1) + 1);
}