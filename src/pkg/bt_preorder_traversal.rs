use crate::pkg::binary_tree_inorder_traversal::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() {
        return vec![];
    }

    let now_root = root.unwrap();
    let mut answer = vec![now_root.borrow().val];
    answer.append(&mut preorder_traversal(now_root.borrow_mut().left.take()));
    answer.append(&mut preorder_traversal(now_root.borrow_mut().right.take()));

    answer
}