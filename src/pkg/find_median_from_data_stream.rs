use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[allow(dead_code)]
struct MedianFinder {
    small: BinaryHeap<i32>,
    big: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            small: BinaryHeap::new(),
            big: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.small.push(num);
        self.big.push(Reverse(self.small.pop().unwrap()));

        if self.big.len() > self.small.len() {
            self.small.push(self.big.pop().unwrap().0)
        }
    }

    fn find_median(&self) -> f64 {
        if self.small.len() == self.big.len() {
            return 0.5 * (self.small.peek().unwrap() + self.big.peek().unwrap().0) as f64;
        }
        *self.small.peek().unwrap() as f64
    }
}