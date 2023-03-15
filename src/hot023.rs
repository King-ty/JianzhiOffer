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

// 题解法为ListNode实现了PartialOrd，可以重复利用节点，减少空间和时间
impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();
        for i in 0..lists.len() {
            if let Some(node) = lists[i].as_mut() {
                let temp = node.next.take();
                heap.push((Reverse(node.val), i));
                lists[i] = temp;
            }
        }
        let mut dummy_root = ListNode::new(0);
        let mut cur = &mut dummy_root;
        while let Some((rev_val, i)) = heap.pop() {
            cur.next = Some(Box::new(ListNode::new(rev_val.0)));
            cur = cur.next.as_mut().unwrap();
            if let Some(node) = lists[i].as_mut() {
                let temp = node.next.take();
                heap.push((Reverse(node.val), i));
                lists[i] = temp;
            }
        }
        dummy_root.next
    }
}
