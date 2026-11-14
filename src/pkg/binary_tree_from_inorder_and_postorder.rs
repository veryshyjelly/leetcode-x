use crate::pkg::binary_tree_inorder_traversal::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn build_tree(inorder: Vec<i32>, mut postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    build_tree_helper(&inorder, &mut postorder)
}

pub fn build_tree_helper(
    inorder: &[i32],
    postorder: &mut Vec<i32>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if inorder.is_empty() {
        return None;
    }

    let root_value = postorder.pop()?;
    let index = inorder.iter().position(|&x| x == root_value).unwrap();

    let left_part = &inorder[..index];
    let right_part = &inorder[index + 1..];

    let mut root = TreeNode::new(root_value);
    root.right = build_tree_helper(&right_part, postorder);
    root.left = build_tree_helper(&left_part, postorder);

    Some(Rc::new(RefCell::new(root)))
}