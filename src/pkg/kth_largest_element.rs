#[allow(dead_code)]
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let mut queue = BinaryHeap::new();
    for n in nums {
        queue.push(Reverse(n));
        if queue.len() > k as usize {
            queue.pop();
        }
    }

    queue.pop().unwrap().0
}