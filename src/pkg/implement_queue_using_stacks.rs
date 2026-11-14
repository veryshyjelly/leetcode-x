#[allow(dead_code)]
struct MyQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl MyQueue {
    fn new() -> Self {
        MyQueue {
            stack1: vec![],
            stack2: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.stack2.append(&mut self.stack1);
        self.stack1.push(x);
        self.stack1.append(&mut self.stack2);
    }

    fn pop(&mut self) -> i32 {
        self.stack1.pop().unwrap()
    }

    fn peek(&self) -> i32 {
        *self.stack1.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack1.is_empty()
    }
}