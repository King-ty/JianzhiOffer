use std::vec;

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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut st1, mut st2) = (vec![], vec![]);
        let mut p = l1;
        while let Some(node) = p {
            st1.push(node.val);
            p = node.next;
        }
        p = l2;
        while let Some(node) = p {
            st2.push(node.val);
            p = node.next;
        }
        let mut c = 0;
        let mut ret = None;
        while !st1.is_empty() && !st2.is_empty() {
            let (a, b) = (st1.pop().unwrap(), st2.pop().unwrap());
            c += a + b;
            ret = Some(Box::new(ListNode {
                val: c % 10,
                next: ret,
            }));
            c /= 10;
        }
        while !st1.is_empty() {
            let a = st1.pop().unwrap();
            c += a;
            ret = Some(Box::new(ListNode {
                val: c % 10,
                next: ret,
            }));
            c /= 10;
        }
        while !st2.is_empty() {
            let a = st2.pop().unwrap();
            c += a;
            ret = Some(Box::new(ListNode {
                val: c % 10,
                next: ret,
            }));
            c /= 10;
        }
        if c > 0 {
            ret = Some(Box::new(ListNode { val: c, next: ret }));
        }

        ret
    }
}
