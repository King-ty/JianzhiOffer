// Rust写链表真的很难绷！！！
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

// impl Solution {
//     pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
//         let (mut fast, mut slow) = (&head, &head);
//         for _ in 0..n {
//             fast = &fast.as_ref().unwrap().next;
//         }
//         while let Some(_) = fast {
//             fast = &fast.as_ref().unwrap().next;
//             slow = &slow.as_ref().unwrap().next;
//         }
//         let mut slow = &mut *slow;
//         *slow = slow.as_ref().unwrap().next;
//         head
//     }
// }

// impl Solution {
//     pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
//         let temp = Some(Box::new(ListNode { val: 0, next: head }));
//         let (mut fast, mut slow) = (&temp, &mut temp.clone());
//         for _ in 0..=n {
//             fast = &fast.as_ref().unwrap().next;
//         }
//         while fast.is_some() {
//             fast = &fast.as_ref().unwrap().next;
//             slow = &mut slow.as_mut().unwrap().next;
//         }
//         let next = &mut slow.as_mut().unwrap().next.as_mut().unwrap().next;
//         slow.as_mut().unwrap().next = next.take();
//         temp.unwrap().next
//     }
// }

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut temp = Some(Box::new(ListNode { val: 0, next: head }));
        let mut slow = &mut temp;
        let mut fast = &mut slow.clone();
        for _ in 0..n {
            fast = &mut fast.as_mut().unwrap().next;
        }
        while fast.is_some() {
            fast = &mut fast.as_mut().unwrap().next;
            slow = &mut slow.as_mut().unwrap().next;
        }
        let next = &mut slow.as_mut().unwrap().next.as_mut().unwrap().next;
        slow.as_mut().unwrap().next = next.take();
        temp.unwrap().next
    }
}
