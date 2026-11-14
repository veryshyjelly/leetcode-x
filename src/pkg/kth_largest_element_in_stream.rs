use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[allow(dead_code)]
struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    k: i32,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut heap = BinaryHeap::new();
        for n in nums {
            heap.push(Reverse(n));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        KthLargest { heap, k }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.k as usize {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}