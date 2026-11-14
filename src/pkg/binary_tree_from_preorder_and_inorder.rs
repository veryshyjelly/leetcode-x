use crate::pkg::binary_tree_inorder_traversal::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn build_tree(mut preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    preorder.reverse();
    build_tree_helper(&mut preorder, &inorder)
}

pub fn build_tree_helper(
    preorder: &mut Vec<i32>,
    inorder: &[i32],
) -> Option<Rc<RefCell<TreeNode>>> {
    if inorder.is_empty() {
        return None;
    }

    let root_value = preorder.pop()?;
    let index = inorder.iter().position(|&x| x == root_value).unwrap();

    let left_part = &inorder[..index];
    let right_part = &inorder[index + 1..];

    let mut root = TreeNode::new(root_value);
    root.left = build_tree_helper(preorder, &left_part);
    root.right = build_tree_helper(preorder, &right_part);

    Some(Rc::new(RefCell::new(root)))
}