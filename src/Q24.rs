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
pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut ret = None;
        // let next = head;
        while let Some(mut node) = head.take() {
            let temp = node.next.take();
            node.next = ret;
            ret = Some(node);
            head = temp;
        }

        ret
        // match head {
        //     Some(mut node) => match Self::reverse_list(node.next.take()) {
        //         Some(mut ret) => {
        //             ret.next = Some(node);
        //             println!("{:?}", ret);
        //             Some(ret)
        //         }
        //         None => Some(node),
        //     },
        //     None => None,
        // }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}
