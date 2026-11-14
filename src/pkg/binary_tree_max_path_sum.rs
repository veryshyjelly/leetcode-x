use crate::pkg::binary_tree_inorder_traversal::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let answer = max_path_sum_helper(root);
    i32::max(answer.0, answer.1)
}

pub fn max_path_sum_helper(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    if root.is_none() {
        return (-5000, -5000);
    }

    let now_root = root.unwrap();
    let now_value = now_root.borrow().val;
    let left_subtree = now_root.borrow_mut().left.take();
    let right_subtree = now_root.borrow_mut().right.take();

    let left_answer = max_path_sum_helper(left_subtree);
    let right_answer = max_path_sum_helper(right_subtree);

    let taking_answer = i32::max(
        now_value,
        now_value + i32::max(left_answer.0, right_answer.0),
    );
    let mut not_taking_answer = now_value;
    if left_answer.0 > 0 {
        not_taking_answer += left_answer.0;
    }
    if right_answer.0 > 0 {
        not_taking_answer += right_answer.0;
    }

    not_taking_answer = i32::max(not_taking_answer, i32::max(left_answer.1, right_answer.1));
    (taking_answer, not_taking_answer)
}