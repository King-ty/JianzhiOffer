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
    pub fn mirror_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let node_ref = node.borrow();
            let ret = Rc::new(RefCell::new(TreeNode::new(node_ref.val)));
            let mut ret_mut = ret.borrow_mut();
            ret_mut.right = Self::mirror_tree(node.borrow().left.clone());
            ret_mut.left = Self::mirror_tree(node.borrow().right.clone());
            drop(ret_mut);
            Some(ret)
        } else {
            None
        }
    }
}
