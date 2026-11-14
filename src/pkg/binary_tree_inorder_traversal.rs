use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[allow(dead_code)]
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() {
        return vec![];
    }

    let now_root = root.unwrap();
    let mut answer = inorder_traversal(now_root.as_ref().borrow_mut().left.take());
    answer.push(now_root.as_ref().borrow_mut().val);
    answer.append(&mut inorder_traversal(
        now_root.as_ref().borrow_mut().right.take(),
    ));
    answer
}