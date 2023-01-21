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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::cmp::max;
        match root {
            Some(node) => {
                let mut node_ref = node.borrow_mut();
                // let mut node_ref: RefMut<TreeNode> = node.borrow_mut();
                return max(
                    Self::max_depth(node_ref.left.take()),
                    Self::max_depth(node_ref.right.take()),
                ) + 1;
            }
            None => return 0,
        }
    }
}
