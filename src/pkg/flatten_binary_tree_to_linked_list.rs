use crate::pkg::binary_tree_inorder_traversal::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    let mut node = root.clone();

    while let Some(n) = node {
        let mut n = n.borrow_mut();
        if let Some(mut cur) = n.left.clone() {
            loop {
                let right = cur.borrow().right.clone();
                if let Some(r) = right {
                    cur = r;
                } else {
                    break;
                }
            }
            let mut b = cur.borrow_mut();
            b.right = n.right.take();
            n.right = n.left.take();
        }
        node = n.right.clone();
    }
}