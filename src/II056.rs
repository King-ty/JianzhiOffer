// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        use std::cmp::Ordering;
        let mut cur = root;
        let mut st = vec![];
        let mut nums = vec![];
        while !st.is_empty() || cur.is_some() {
            while let Some(node) = cur {
                cur = node.borrow_mut().left.take();
                st.push(node);
            }
            if let Some(node) = st.pop() {
                let mut node_ref = node.borrow_mut();
                nums.push(node_ref.val);
                if node_ref.right.is_some() {
                    cur = node_ref.right.take();
                }
            }
        }
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            match (nums[l] + nums[r]).cmp(&k) {
                Ordering::Equal => return true,
                Ordering::Less => l += 1,
                Ordering::Greater => r -= 1,
            };
        }
        false
    }
}
