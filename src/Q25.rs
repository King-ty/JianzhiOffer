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
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut ret = Some(Box::new(ListNode { val: 0, next: None }));
        let mut cur = ret.as_mut();
        while l1.is_some() && l2.is_some() {
            let val1 = l1.as_ref().unwrap().val;
            let val2 = l2.as_ref().unwrap().val;
            if val1 < val2 {
                cur.as_mut().unwrap().next = l1;
                cur = cur.unwrap().next.as_mut();
                l1 = cur.as_mut().unwrap().next.take();
            } else {
                cur.as_mut().unwrap().next = l2;
                cur = cur.unwrap().next.as_mut();
                l2 = cur.as_mut().unwrap().next.take();
            }
            // if ret.is_none() {
            //     ret = &mut tail.as_mut();
            // }
        }
        if l1.is_some() {
            cur.as_mut().unwrap().next = l1;
        } else {
            cur.as_mut().unwrap().next = l2;
        }
        ret.unwrap().next
    }
}
