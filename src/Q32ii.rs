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
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut nodes = VecDeque::new();
        let mut ret = Vec::new();
        let mut layer_tot = 0;
        if let Some(node) = root {
            nodes.push_back((node, 0));
        }
        while let Some((node, layer)) = nodes.pop_front() {
            if layer >= layer_tot {
                ret.push(Vec::new());
                layer_tot += 1;
            }
            let mut node_mut = node.borrow_mut();
            ret[layer].push(node_mut.val);
            if let Some(node) = node_mut.left.take() {
                nodes.push_back((node, layer + 1));
            }
            if let Some(node) = node_mut.right.take() {
                nodes.push_back((node, layer + 1));
            }
        }
        ret
    }
}
