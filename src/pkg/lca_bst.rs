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
    lca_helper(root, i32::min(p, q), i32::max(p, q))
}

pub fn lca_helper(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: i32,
    q: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    let now_root = root.unwrap();
    let now_value = now_root.borrow().val;

    if now_value == p || now_value == q || (now_value > p && now_value < q) {
        Some(now_root)
    } else if now_value > q {
        lca_helper(now_root.borrow_mut().left.take(), p, q)
    } else {
        lca_helper(now_root.borrow_mut().right.take(), p, q)
    }
}