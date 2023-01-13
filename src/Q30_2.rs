use std::cmp::min;

struct MinStack {
    data: Vec<i32>,
    min: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            data: Vec::new(),
            min: vec![i32::MAX],
        }
    }

    fn push(&mut self, x: i32) {
        self.data.push(x);
        self.min.push(min(self.min[self.min.len() - 1], x));
    }

    fn pop(&mut self) {
        self.data.pop();
        self.min.pop();
    }

    fn top(&self) -> i32 {
        self.data[self.data.len() - 1]
    }

    fn min(&self) -> i32 {
        self.min[self.min.len() - 1]
    }
}
