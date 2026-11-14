use std::collections::VecDeque;

struct MyStack {
    q1: VecDeque<i32>,
    q2: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        MyStack {
            q1: VecDeque::new(),
            q2: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        while let Some(q) = self.q1.pop_front() {
            self.q2.push_back(q);
        }
        self.q1.push_back(x);
        while let Some(q) = self.q2.pop_front() {
            self.q1.push_back(q);
        }
    }

    fn pop(&mut self) -> i32 {
        self.q1.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        self.q1[0]
    }

    fn empty(&self) -> bool {
        self.q1.is_empty()
    }
}