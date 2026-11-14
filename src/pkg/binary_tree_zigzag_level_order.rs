use crate::pkg::binary_tree_inorder_traversal::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() {
        return vec![];
    }

    let mut next_level = vec![root.unwrap()];
    let mut answer = vec![];

    let mut zig_zag = false;

    while next_level.len() != 0 {
        let current_level = next_level;
        next_level = vec![];
        let mut current_answer = vec![];
        for node in current_level {
            current_answer.push(node.as_ref().borrow().val);
            if let Some(left_subtree) = node.as_ref().borrow_mut().left.take() {
                next_level.push(left_subtree);
            }
            if let Some(right_subtree) = node.as_ref().borrow_mut().right.take() {
                next_level.push(right_subtree);
            }
        }
        if zig_zag {
            current_answer.reverse();
        }
        zig_zag = !zig_zag;
        answer.push(current_answer);
    }

    answer
}