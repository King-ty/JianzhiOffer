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

pub struct Solution;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut nodes = VecDeque::new();
        let mut ret = Vec::new();
        nodes.push_back(root);
        while let Some(current) = nodes.pop_front() {
            if let Some(node) = current {
                let mut node_mut = node.borrow_mut();
                ret.push(node_mut.val);
                nodes.push_back(node_mut.left.take());
                nodes.push_back(node_mut.right.take());
            }
        }
        ret
    }
}
