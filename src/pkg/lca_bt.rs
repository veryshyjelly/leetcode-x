use crate::pkg::binary_tree_inorder_traversal::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let p = p.unwrap().borrow().val;
    let q = q.unwrap().borrow().val;
    lca_helper(root, p, q)
}

pub fn lca_helper(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: i32,
    q: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return None;
    }
    let now_root = root.unwrap();
    if now_root.borrow().val == p || now_root.borrow().val == q {
        return Some(now_root);
    }
    let left_answer = lca_helper(now_root.borrow_mut().left.take(), p, q);
    let right_answer = lca_helper(now_root.borrow_mut().right.take(), p, q);

    if left_answer.is_some() && right_answer.is_some() {
        Some(now_root)
    } else if left_answer.is_some() {
        left_answer
    } else {
        right_answer
    }
}