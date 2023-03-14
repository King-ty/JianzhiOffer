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

// 更优美的写法
impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_root = ListNode::new(0);
        let mut cur = &mut dummy_root;
        while let (Some(node1), Some(node2)) = (list1.as_ref(), list2.as_ref()) {
            if node1.val < node2.val {
                cur.next = list1;
                cur = cur.next.as_mut().unwrap();
                list1 = cur.next.take();
            } else {
                cur.next = list2;
                cur = cur.next.as_mut().unwrap();
                list2 = cur.next.take();
            }
        }
        cur.next = list1.or(list2);
        dummy_root.next
    }
}

// 我自己的写法，不够优美
impl Solution {
    pub fn merge_two_lists_mine(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_root = Some(Box::new(ListNode::new(0)));
        let mut cur = &mut dummy_root;
        let (mut p1, mut p2) = (list1, list2);
        while p1.is_some() && p2.is_some() {
            let mut temp;
            if p1.as_ref().unwrap().val < p2.as_ref().unwrap().val {
                temp = p1.take();
                p1 = temp.as_mut().unwrap().next.take();
            } else {
                temp = p2.take();
                p2 = temp.as_mut().unwrap().next.take();
            }
            cur.as_mut().unwrap().next = temp;
            cur = &mut cur.as_mut().unwrap().next;
        }
        if p1.is_some() {
            cur.as_mut().unwrap().next = p1;
        } else {
            cur.as_mut().unwrap().next = p2;
        }
        dummy_root.unwrap().next
    }
}
