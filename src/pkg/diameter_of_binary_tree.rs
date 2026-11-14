use crate::pkg::binary_tree_inorder_traversal::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    diameter_helper(root).1
}

fn diameter_helper(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    if root.is_none() {
        return (0, 0);
    }

    let now_root = root.unwrap();
    let left_answer = diameter_helper(now_root.borrow_mut().left.take());
    let right_answer = diameter_helper(now_root.borrow_mut().right.take());
    (
        i32::max(left_answer.0, right_answer.0) + 1,
        i32::max(
            left_answer.0 + right_answer.0 + 1,
            i32::max(left_answer.1, right_answer.1),
        ),
    )
}