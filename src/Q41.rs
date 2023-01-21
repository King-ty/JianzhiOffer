// 对顶堆法
use std::{cmp::Reverse, collections::BinaryHeap, iter::Rev};

struct MedianFinder {
    heap_max: BinaryHeap<Reverse<i32>>,
    heap_min: BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            heap_max: BinaryHeap::new(),
            heap_min: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.heap_min.is_empty() || *self.heap_min.peek().unwrap() > num {
            self.heap_min.push(num);
            if self.heap_min.len() > self.heap_max.len() + 1 {
                self.heap_max.push(Reverse(self.heap_min.pop().unwrap()));
            }
        } else {
            self.heap_max.push(Reverse(num));
            if self.heap_max.len() > self.heap_min.len() {
                self.heap_min.push(self.heap_max.pop().unwrap().0);
            }
        }
    }

    fn find_median(&self) -> f64 {
        if self.heap_min.len() > self.heap_max.len() {
            *self.heap_min.peek().unwrap() as f64
        } else {
            (*self.heap_min.peek().unwrap() + self.heap_max.peek().unwrap().0) as f64 / 2.0
        }
    }
}
