use crate::pkg::binary_tree_inorder_traversal::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() {
        return vec![];
    }

    let now_root = root.unwrap();
    let mut answer = vec![];
    answer.append(&mut postorder_traversal(now_root.borrow_mut().left.take()));
    answer.append(&mut postorder_traversal(now_root.borrow_mut().right.take()));
    answer.push(now_root.borrow().val);

    answer
}