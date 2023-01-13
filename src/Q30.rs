struct MinStack {
    data: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        self.data.push(x);
    }

    fn pop(&mut self) {
        self.data.pop();
    }

    fn top(&self) -> i32 {
        self.data[self.data.len() - 1]
    }

    fn min(&self) -> i32 {
        let mut min = self.data[0];
        for value in self.data.iter() {
            if *value < min {
                min = *value;
            }
        }
        min
    }
}
