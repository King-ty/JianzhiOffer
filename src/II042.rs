use std::collections::LinkedList;

struct RecentCounter {
    qq: LinkedList<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        Self {
            qq: LinkedList::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        while let Some(q) = self.qq.front() {
            if t - q > 3000 {
                self.qq.pop_front();
            } else {
                break;
            }
        }
        self.qq.push_back(t);
        self.qq.len() as i32
    }
}
