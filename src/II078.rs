// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;
// 堆方法，也可以归并merge，时间复杂度相同，空间好一点
impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();
        for i in 0..lists.len() {
            if let Some(node) = lists[i].as_mut() {
                heap.push((Reverse(node.val), i));
                lists[i] = node.next.take();
            }
        }
        let mut dummy_head = ListNode::new(0);
        let mut cur = &mut dummy_head.next;
        while let Some((val, i)) = heap.pop() {
            if let Some(node) = lists[i].as_mut() {
                heap.push((Reverse(node.val), i));
                lists[i] = node.next.take();
            }
            *cur = Some(Box::new(ListNode::new(val.0)));
            cur = &mut (*cur).as_mut().unwrap().next;
        }
        dummy_head.next
    }
}
