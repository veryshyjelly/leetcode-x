use crate::pkg::binary_tree_inorder_traversal::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct BSTIterator {
    stack: Vec<Option<Rc<RefCell<TreeNode>>>>,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        BSTIterator { stack: vec![root] }
    }

    fn next(&mut self) -> i32 {
        if !self.has_next() {
            return -1;
        }
        let mut top = self.stack.pop().unwrap();
        while !is_leaf(&top) {
            let left_child = top.as_mut().unwrap().borrow_mut().left.take();
            let right_child = top.as_mut().unwrap().borrow_mut().right.take();
            if right_child.is_some() {
                self.stack.push(right_child);
            }
            self.stack.push(top);
            if left_child.is_some() {
                self.stack.push(left_child);
            }
            top = self.stack.pop().unwrap();
        }
        top.unwrap().borrow_mut().val
    }

    fn has_next(&self) -> bool {
        self.stack.len() == 0
    }
}

fn is_leaf(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    root.as_ref().unwrap().borrow().left.is_none()
        && root.as_ref().unwrap().borrow().right.is_none()
}