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

impl Solution {
    pub fn get_kth_from_end(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut cur = &mut head;
        let mut tot = 0;
        while cur.is_some() {
            tot += 1;
            cur = &mut cur.as_mut().unwrap().next;
        }
        let k = tot - k;
        cur = &mut head;
        for _ in 0..k {
            cur = &mut cur.as_mut().unwrap().next;
        }
        cur.take()
    }
}
