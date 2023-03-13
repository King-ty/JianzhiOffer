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

// unsafe快慢指针
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy_root = Box::new(ListNode { val: 0, next: head });
        let (mut fast, mut slow) = (
            &mut dummy_root as *mut Box<ListNode>,
            &mut dummy_root as *mut Box<ListNode>,
        );
        unsafe {
            for _ in 0..n {
                fast = (*fast).next.as_mut().unwrap();
            }
            while (*fast).next.is_some() {
                fast = (*fast).next.as_mut().unwrap();
                slow = (*slow).next.as_mut().unwrap();
            }
            (*slow).next = (*slow).next.as_mut().unwrap().next.take();
        }
        dummy_root.next
    }
}

// impl Solution {
//     pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
//         let (mut fast, mut slow) = (&head, &head);
//         for _ in 0..=n {
//             fast = &fast.as_ref().unwrap().next;
//         }
//         while fast.is_some() {
//             fast = &fast.as_ref().unwrap().next;
//             slow = &slow.as_ref().unwrap().next;
//         }
//         if slow == &head {
//             head.unwrap().next
//         } else {
//             // 没法做了，因为不是mut的
//             head
//         }
//     }
// }
