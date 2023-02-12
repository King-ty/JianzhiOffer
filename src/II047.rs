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
    pub fn prune_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.as_mut() {
            let mut node_mut = node.borrow_mut();
            node_mut.left = Self::prune_tree(node_mut.left.take());
            node_mut.right = Self::prune_tree(node_mut.right.take());
            if node_mut.val == 0 && node_mut.left.is_none() && node_mut.right.is_none() {
                return None;
            }
        }
        root
    }
}
