#[allow(dead_code)]
struct MinStack {
    val: Vec<i32>,
    min: Vec<i32>,
}

#[allow(dead_code)]
impl MinStack {
    fn new() -> Self {
        MinStack {
            val: vec![],
            min: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.val.push(val);
        self.min
            .push(i32::min(*self.min.last().unwrap_or(&i32::MAX), val));
    }

    fn pop(&mut self) {
        self.val.pop();
        self.min.pop();
    }

    fn top(&self) -> i32 {
        *self.val.last().unwrap_or(&i32::MAX)
    }

    fn get_min(&self) -> i32 {
        *self.min.last().unwrap_or(&i32::MAX)
    }
}