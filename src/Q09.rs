struct CQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

/**
* `&self` means the method takes an immutable reference.
* If you need a mutable reference, change it to `&mut self` instead.
*/
impl CQueue {
    fn new() -> Self {
        Self {
            stack1: Vec::new(),
            stack2: Vec::new(),
        }
    }

    fn append_tail(&mut self, value: i32) {
        self.stack1.push(value);
    }

    fn delete_head(&mut self) -> i32 {
        match self.stack2.pop() {
            Some(value) => value,
            None => match self.stack1.is_empty() {
                true => -1,
                false => {
                    while !self.stack1.is_empty() {
                        self.stack2.push(self.stack1.pop().unwrap());
                    }
                    self.stack2.pop().unwrap()
                }
            },
        }
    }
}
