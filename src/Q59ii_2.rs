use std::collections::{LinkedList, VecDeque};
struct MaxQueue {
    mq: VecDeque<i32>,
    qq: LinkedList<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MaxQueue {
    fn new() -> Self {
        Self {
            mq: VecDeque::new(),
            qq: LinkedList::new(),
        }
    }

    fn max_value(&self) -> i32 {
        match self.mq.front() {
            Some(val) => *val,
            None => -1,
        }
    }

    fn push_back(&mut self, value: i32) {
        self.qq.push_back(value);
        while !self.mq.is_empty() && *self.mq.back().unwrap() < value {
            self.mq.pop_back();
        }
        self.mq.push_back(value);
    }

    fn pop_front(&mut self) -> i32 {
        let ret = match self.qq.pop_front() {
            Some(val) => val,
            None => return -1,
        };
        if *self.mq.front().unwrap() == ret {
            self.mq.pop_front();
        }
        ret
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_Q59ii() {
        use super::MaxQueue;
        let mut max_q = MaxQueue::new();
        max_q.pop_front();
    }
}

// ["MaxQueue","max_value","pop_front","max_value","push_back","max_value","pop_front","max_value","pop_front","push_back","pop_front","pop_front","pop_front","push_back","pop_front","max_value","pop_front","max_value","push_back","push_back","max_value","push_back","max_value","max_value","max_value","push_back","pop_front","max_value","push_back","max_value","max_value","max_value","pop_front","push_back","push_back","push_back","push_back","pop_front","pop_front","max_value","pop_front","pop_front","max_value","push_back","push_back","pop_front","push_back","push_back","push_back","push_back","pop_front","max_value","push_back","max_value","max_value","pop_front","max_value","max_value","max_value","push_back","pop_front","push_back","pop_front","max_value","max_value","max_value","push_back","pop_front","push_back","push_back","push_back","pop_front","max_value","pop_front","max_value","max_value","max_value","pop_front","push_back","pop_front","push_back","push_back","pop_front","push_back","pop_front","push_back","pop_front","pop_front","push_back","pop_front","pop_front","pop_front","push_back","push_back","max_value","push_back","pop_front","push_back","push_back","pop_front"]
// [[],[],[],[],[46],[],[],[],[],[868],[],[],[],[525],[],[],[],[],[123],[646],[],[229],[],[],[],[871],[],[],[285],[],[],[],[],[45],[140],[837],[545],[],[],[],[],[],[],[561],[237],[],[633],[98],[806],[717],[],[],[186],[],[],[],[],[],[],[268],[],[29],[],[],[],[],[866],[],[239],[3],[850],[],[],[],[],[],[],[],[310],[],[674],[770],[],[525],[],[425],[],[],[720],[],[],[],[373],[411],[],[831],[],[765],[701],[]]
