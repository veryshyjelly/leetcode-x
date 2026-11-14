use crate::pkg::binary_tree_inorder_traversal::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    sorted_to_bst_helper(&nums)
}

pub fn sorted_to_bst_helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        return None;
    }

    let index = nums.len() / 2;
    let left_part = &nums[..index];
    let right_part = &nums[index + 1..];

    let mut root = TreeNode::new(nums[index]);
    root.left = sorted_to_bst_helper(left_part);
    root.right = sorted_to_bst_helper(right_part);

    Some(Rc::new(RefCell::new(root)))
}