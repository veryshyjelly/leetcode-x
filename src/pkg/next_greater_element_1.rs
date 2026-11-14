use std::collections::HashMap;

#[allow(dead_code)]
pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut next_greater = HashMap::new();
    let mut stack: Vec<i32> = vec![];
    for n in nums2.into_iter().rev() {
        while stack.len() != 0 && n.ge(stack.last().unwrap()) {
            stack.pop();
        }
        if stack.len() == 0 {
            next_greater.insert(n, -1);
        } else {
            next_greater.insert(n, *stack.last().unwrap());
        }
        stack.push(n);
    }
    nums1
        .into_iter()
        .map(|x| *next_greater.get(&x).unwrap())
        .collect()
}